//! Player module: ship control, energy, targeting, camera, and HUD.

use bevy::prelude::*;

pub mod camera;
pub mod constants;
pub mod energy;
pub mod hud;
pub mod indicators;
pub mod input;
pub mod ship_template;
pub mod weapon_panel;

use constants::energy as energy_consts;

// ---------------------------------------------------------------------------
// Marker & core components
// ---------------------------------------------------------------------------

/// Marker component for the player-controlled ship. Exactly one entity has this.
#[derive(Component)]
pub struct Player;

/// Reactor capacity of the player ship.
#[derive(Component)]
pub struct ShipReactor {
    pub max_power: u32,
}

/// Base stats stored at spawn so the energy system can scale from them.
#[derive(Component)]
pub struct PlayerBaseStats {
    pub thrust: f32,
    pub max_turn_speed: f32,
}

/// Placed on each weapon child of the player ship.
#[derive(Component)]
pub struct WeaponSlot {
    /// Ordering index (lower = powered first).
    pub index: u32,
    /// How many energy cells this weapon needs.
    pub power_cost: u32,
    /// Whether the weapon currently has power.
    pub powered: bool,
}

/// Mouse-button group a player turret responds to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerWeaponGroup {
    /// Left-click: lasers, flak — anything that fires immediately.
    Primary,
    /// Right-click: missiles. Requires a locked target.
    Missile,
}

/// Marker placed on every player turret. Carries the input group it listens to.
/// Used to opt player turrets out of the generic auto-fire system in combat::tools.
#[derive(Component)]
pub struct PlayerWeapon {
    pub group: PlayerWeaponGroup,
}

/// Present on Primary turrets that fire ballistic projectiles instead of hitscan.
/// Absence on a Primary turret means it uses hitscan (lasers/flak).
#[derive(Component, Copy, Clone)]
pub struct BallisticConfig {
    pub bullet_speed: f32,
    /// Random aim jitter in radians, applied symmetrically (±spread).
    pub spread: f32,
    pub damage: f32,
    pub accuracy: f32,
}

/// Display label for a weapon turret. Shown in the weapon panel UI.
#[derive(Component, Clone)]
pub struct WeaponName(pub &'static str);

/// User-controlled enable flag for a turret. When false, the turret will not be
/// armed (and therefore won't fire) regardless of its power state. Toggled by
/// the weapon panel UI.
#[derive(Component, Copy, Clone)]
pub struct WeaponEnabled(pub bool);

/// Ammunition for weapons that consume ammo (e.g. missiles).
#[derive(Component)]
pub struct Ammunition {
    pub current: u32,
    pub max: u32,
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

/// Which entity the player has selected as target.
#[derive(Resource, Default)]
pub struct PlayerTarget(pub Option<Entity>);

/// Is the game paused (tactical pause)?
#[derive(Resource)]
pub struct GamePaused(pub bool);

impl Default for GamePaused {
    fn default() -> Self {
        GamePaused(false)
    }
}

/// Power allocation across the three core systems.
#[derive(Resource)]
pub struct SystemPower {
    pub shields: PowerAllocation,
    pub weapons: PowerAllocation,
    pub engines: PowerAllocation,
}

#[derive(Clone, Copy)]
pub struct PowerAllocation {
    pub current: u32,
    pub max_level: u32,
}

impl Default for SystemPower {
    fn default() -> Self {
        SystemPower {
            shields: PowerAllocation {
                current: energy_consts::DEFAULT_SHIELD_POWER,
                max_level: energy_consts::SHIELD_MAX_LEVEL,
            },
            weapons: PowerAllocation {
                current: energy_consts::DEFAULT_WEAPON_POWER,
                max_level: energy_consts::WEAPON_MAX_LEVEL,
            },
            engines: PowerAllocation {
                current: energy_consts::DEFAULT_ENGINE_POWER,
                max_level: energy_consts::ENGINE_MAX_LEVEL,
            },
        }
    }
}

/// Whether weapons fire automatically when a target is locked.
/// Off by default — combat is now driven by mouse buttons.
#[derive(Resource, Default)]
pub struct AutoFire(pub bool);

/// Per-frame mouse-button state, sampled from MouseButton::Left/Right.
/// Read by the player firing system to decide which weapon group fires this tick.
#[derive(Resource, Default)]
pub struct PlayerFireInput {
    pub primary_held: bool,
    pub missile_held: bool,
}

/// World-space cursor position. None when cursor is offscreen or no window.
/// Updated each frame from window cursor + camera viewport_to_world_2d.
#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Option<Vec2>);

/// Mouse-wheel-driven camera zoom. Smaller scale = zoomed in.
#[derive(Resource)]
pub struct CameraZoom {
    pub target_scale: f32,
}

impl Default for CameraZoom {
    fn default() -> Self {
        CameraZoom {
            target_scale: constants::controls::CAMERA_ZOOM_DEFAULT,
        }
    }
}

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.init_resource::<PlayerTarget>();
        app.init_resource::<GamePaused>();
        app.init_resource::<SystemPower>();
        app.init_resource::<AutoFire>();
        app.init_resource::<PlayerFireInput>();
        app.init_resource::<CursorWorldPos>();
        app.init_resource::<CameraZoom>();

        // Ship template plugin (spawning)
        app.add_plugins(ship_template::PlayerShipTemplatePlugin);

        // Targeting indicators (lock ring + hover ring)
        app.add_plugins(indicators::IndicatorsPlugin);

        // Weapon panel UI (top-right, below target info)
        app.add_plugins(weapon_panel::WeaponPanelPlugin);

        // Input systems (run every frame in Update)
        app.add_systems(
            Update,
            (
                (
                    input::player_movement_input,
                    input::player_pause_input,
                    input::player_speed_input,
                    input::player_zoom_input,
                ),
                (
                    input::player_energy_input,
                    input::player_auto_fire_input,
                    input::player_lock_target_input,
                    input::player_mouse_fire_input,
                    input::update_cursor_world_pos,
                    input::clear_dead_target_lock,
                ),
            ),
        );

        // Camera follow (PostUpdate, after transform propagation)
        app.add_systems(PostUpdate, camera::camera_follow_player);

        // Energy effects (FixedUpdate, before combat systems)
        app.add_systems(FixedUpdate, energy::apply_energy_to_stats);

        // Missiles use the standard Effector path; runs before tools_activate_effectors
        // so its tool.firing flag is consumed this tick.
        app.add_systems(
            FixedUpdate,
            input::player_fire_missiles
                .after(crate::combat::tools::update_cooldowns)
                .after(crate::combat::targets::copy_targets_from_parents)
                .before(crate::combat::tools::tools_activate_effectors),
        );

        // Primary weapons use cursor hitscan and bypass the Effector path —
        // they spawn the effect entity manually so they can target enemies
        // chosen by ray rather than the inherited PlayerTarget. Run before
        // determine_missed_attacks so the manual Miss result sticks.
        app.add_systems(
            FixedUpdate,
            input::player_primary_fire
                .after(crate::combat::tools::update_cooldowns)
                .before(crate::combat::evasion::determine_missed_attacks),
        );

        // HUD
        app.add_systems(Startup, hud::spawn_hud);
        app.add_systems(
            Update,
            (
                hud::update_health_bar,
                hud::update_shield_bar,
                hud::update_energy_bars,
                hud::update_target_info,
                hud::update_game_speed_text,
            ),
        );
    }
}
