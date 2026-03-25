//! Player ship template and spawning.

use bevy::prelude::*;

use crate::{
    combat::{
        damage::LastDamageTimer,
        effects::Effector,
        evasion::Evasion,
        mortal::{Health, MaxHealth, Mortal},
        projectile::CircularHitBox,
        shields::{MaxShieldHP, Shield},
        targets::InheritTargetFromParent,
        tools::{Cooldown, TargettedTool},
        Target, Team,
    },
    fx::{animated::AnimatedEffects, death::DeathEffect},
    materials::ShipMaterial,
    movement::{Mass, MaxTurnSpeed, MovementBundle, Thrust},
    templates::ships::spawn::{spawn_ships_and_despawn_spawn_commands, SpawnShipTemplate},
    templates::weapons::{pulse_laser_attack, small_rocket_attack},
};

use super::constants::player_ship::*;
use super::constants::weapons::*;
use super::{Ammunition, Player, PlayerBaseStats, ShipReactor, WeaponSlot};
use super::constants::energy::REACTOR_MAX_POWER;

// ---------------------------------------------------------------------------
// Resources (preloaded assets)
// ---------------------------------------------------------------------------

#[derive(Resource)]
pub struct PlayerShipResources {
    color_texture: Handle<Image>,
    mask_texture: Handle<Image>,
    mesh: Handle<Mesh>,
}

// ---------------------------------------------------------------------------
// Spawner
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct PlayerShipSpawner;

impl SpawnShipTemplate for PlayerShipSpawner {
    type Resources<'a> = PlayerShipResources;

    fn spawn(
        &self,
        commands: &mut Commands,
        resources: &Res<PlayerShipResources>,
        materials: &mut ResMut<Assets<ShipMaterial>>,
    ) -> Entity {
        // --- Turret 0: Front-top pulse laser ---
        let turret_0 = commands
            .spawn((
                Transform {
                    translation: Vec3::new(
                        TURRET_FRONT_TOP_OFFSET[0],
                        TURRET_FRONT_TOP_OFFSET[1],
                        0.0,
                    ),
                    ..default()
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                Cooldown::new(PULSE_LASER_COOLDOWN),
                TargettedTool {
                    range: PULSE_LASER_RANGE,
                    cone: PULSE_LASER_CONE,
                    armed: true,
                    firing: false,
                },
                Effector::new(pulse_laser_attack),
            ))
            .insert(WeaponSlot {
                index: 0,
                power_cost: PULSE_LASER_POWER_COST,
                powered: true,
            })
            .id();

        // --- Turret 1: Side-left flak cannon ---
        let turret_1 = commands
            .spawn((
                Transform {
                    translation: Vec3::new(
                        TURRET_SIDE_LEFT_OFFSET[0],
                        TURRET_SIDE_LEFT_OFFSET[1],
                        0.0,
                    ),
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    ..default()
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                Cooldown::new(FLAK_COOLDOWN),
                TargettedTool {
                    range: FLAK_RANGE,
                    cone: FLAK_CONE,
                    armed: true,
                    firing: false,
                },
                Effector::new(pulse_laser_attack), // TODO: replace with flak_attack in V2
            ))
            .insert(WeaponSlot {
                index: 1,
                power_cost: FLAK_POWER_COST,
                powered: true,
            })
            .id();

        // --- Turret 2: Side-right missile launcher ---
        let turret_2 = commands
            .spawn((
                Transform {
                    translation: Vec3::new(
                        TURRET_SIDE_RIGHT_OFFSET[0],
                        TURRET_SIDE_RIGHT_OFFSET[1],
                        0.0,
                    ),
                    rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                    ..default()
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                Cooldown::new(MISSILE_COOLDOWN),
                TargettedTool {
                    range: MISSILE_RANGE,
                    cone: MISSILE_CONE,
                    armed: true,
                    firing: false,
                },
                Effector::new(small_rocket_attack),
            ))
            .insert(WeaponSlot {
                index: 2,
                power_cost: MISSILE_POWER_COST,
                powered: true,
            })
            .insert(Ammunition {
                current: MISSILE_START_AMMO,
                max: MISSILE_MAX_AMMO,
            })
            .id();

        // --- Turret 3: Front-bottom pulse laser ---
        let turret_3 = commands
            .spawn((
                Transform {
                    translation: Vec3::new(
                        TURRET_FRONT_BOTTOM_OFFSET[0],
                        TURRET_FRONT_BOTTOM_OFFSET[1],
                        0.0,
                    ),
                    ..default()
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                Cooldown::new(PULSE_LASER_COOLDOWN),
                TargettedTool {
                    range: PULSE_LASER_RANGE,
                    cone: PULSE_LASER_CONE,
                    armed: true,
                    firing: false,
                },
                Effector::new(pulse_laser_attack),
            ))
            .insert(WeaponSlot {
                index: 3,
                power_cost: PULSE_LASER_POWER_COST,
                powered: true,
            })
            .id();

        // --- Player ship body ---
        commands
            .spawn((
                Mesh2d(resources.mesh.clone()),
                MeshMaterial2d(materials.add(ShipMaterial {
                    color: LinearRgba::new(0.0, 1.0, 0.0, 1.0), // Green for player
                    last_damaged_time: 1.0,
                    base_texture: resources.color_texture.clone(),
                    color_mask: resources.mask_texture.clone(),
                })),
            ))
            .insert(Player)
            .insert(ShipReactor {
                max_power: REACTOR_MAX_POWER,
            })
            .insert(PlayerBaseStats {
                thrust: PLAYER_THRUST,
                max_turn_speed: PLAYER_MAX_TURN_SPEED,
            })
            .insert(MovementBundle {
                max_turn_speed: MaxTurnSpeed::new(PLAYER_MAX_TURN_SPEED),
                mass: Mass(PLAYER_MASS),
                thrust: Thrust(PLAYER_THRUST),
                ..default()
            })
            // NO AI components (no IdleBehavior, TurnToDestinationBehavior, etc.)
            .insert((
                Target::default(),
                Team(1),
                Health(PLAYER_MAX_HEALTH),
                MaxHealth(PLAYER_MAX_HEALTH),
                crate::ai::aggression::AgentCategory::FRIGATE,
                Mortal,
                LastDamageTimer(0.0),
            ))
            .insert(Shield {
                health: PLAYER_MAX_SHIELD_HP,
                radius: PLAYER_SHIELD_RADIUS,
            })
            .insert(MaxShieldHP(PLAYER_MAX_SHIELD_HP))
            .insert(CircularHitBox {
                radius: PLAYER_HITBOX_RADIUS,
            })
            .insert(Evasion::new(PLAYER_EVASION_BASE))
            .insert(DeathEffect {
                time_to_explosion: 0.1,
                time_to_smoke: 0.05,
                dying_explosion: AnimatedEffects::MediumExplosion,
                death_explosion: AnimatedEffects::BigFlashExplosion,
            })
            .add_children(&[turret_0, turret_1, turret_2, turret_3])
            .id()
    }
}

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct PlayerShipTemplatePlugin;

impl PlayerShipTemplatePlugin {
    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        let resources = PlayerShipResources {
            // Reuse the crab (frigate) art for V1
            color_texture: assets.load("art/crab.png"),
            mask_texture: assets.load("art/crab_mask.png"),
            mesh: meshes
                .add(Mesh::from(Rectangle::new(64.0, 64.0))),
        };
        commands.insert_resource(resources);
    }
}

impl Plugin for PlayerShipTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, PlayerShipTemplatePlugin::setup);
        app.add_systems(
            FixedUpdate,
            spawn_ships_and_despawn_spawn_commands::<PlayerShipSpawner>,
        );
    }
}
