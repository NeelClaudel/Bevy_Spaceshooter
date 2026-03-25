//! Player module: ship control, energy, targeting, camera, and HUD.

use bevy::prelude::*;

pub mod camera;
pub mod constants;
pub mod energy;
pub mod hud;
pub mod input;
pub mod ship_template;

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

/// Whether the player has toggled "hold fire" (suppress all weapons).
#[derive(Resource)]
pub struct HoldFire(pub bool);

impl Default for HoldFire {
    fn default() -> Self {
        HoldFire(false)
    }
}

/// Whether turrets automatically target the nearest enemy.
#[derive(Resource)]
pub struct AutoTarget {
    pub enabled: bool,
    /// Timer for periodic re-scanning (avoids switching targets every frame).
    pub retarget_timer: f32,
}

impl Default for AutoTarget {
    fn default() -> Self {
        AutoTarget {
            enabled: true, // ON by default
            retarget_timer: 0.0,
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
        app.init_resource::<HoldFire>();
        app.init_resource::<AutoTarget>();

        // Ship template plugin (spawning)
        app.add_plugins(ship_template::PlayerShipTemplatePlugin);

        // Input systems (run every frame in Update)
        app.add_systems(
            Update,
            (
                input::player_movement_input,
                input::player_target_input,
                input::player_pause_input,
                input::player_speed_input,
                input::player_energy_input,
                input::player_hold_fire_input,
                input::player_auto_target_toggle,
                input::auto_target_nearest_enemy,
            ),
        );

        // Camera follow (PostUpdate, after transform propagation)
        app.add_systems(PostUpdate, camera::camera_follow_player);

        // Energy effects (FixedUpdate, before combat systems)
        app.add_systems(FixedUpdate, energy::apply_energy_to_stats);

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
