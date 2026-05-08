//! Player thruster VFX.
//!
//! Each of the 4 nozzles renders as two layered HDR rectangles — a colored
//! outer flame and a brighter white-hot inner core child — so the flame reads
//! as superheated rather than as a flat slab of color. WASD input drives a
//! smoothed intensity per thruster which scales the flame length and gates
//! particle emission.
//!
//! The forward thruster also emits a particle exhaust trail: small bright
//! quads launched outward along the exhaust direction with a perpendicular
//! spread, shrinking from full size to zero over a short lifetime. Particles
//! live in world space (no parent) so they are left behind as the ship moves.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy::sprite_render::{ColorMaterial, MeshMaterial2d};
use rand::Rng;

use crate::game::GameTimeDelta;
use crate::player::constants::controls::REVERSE_SPEED_FRACTION;
use crate::player::constants::thrusters::*;
use crate::player::{GamePaused, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThrusterDir {
    Forward,
    Reverse,
    StrafeLeft,
    StrafeRight,
}

/// One nozzle. Spawned as a child of the ship so its Transform is
/// ship-local. The `update_thruster_intensity` system rewrites the entity's
/// translation/scale each tick: position = nozzle + exhaust * length/2 so
/// the rendered rectangle's "bottom" sits at the nozzle and grows outward.
#[derive(Component)]
pub struct Thruster {
    pub direction: ThrusterDir,
    /// Anchor in ship-local space (the entity origin slides off this point).
    pub nozzle: Vec2,
    /// Unit vector in ship-local space pointing outward from the nozzle.
    pub exhaust: Vec2,
    pub max_length: f32,
    pub width: f32,
    /// Smoothed intensity in [0, 1]. Drives flame length & particle cadence.
    pub intensity: f32,
    /// Time-since-last-emission accumulator.
    pub emit_timer: f32,
}

/// World-space exhaust particle. Travels along `velocity` and shrinks from
/// `initial_size` to zero over `max_lifetime`, then despawns. Uses scale-only
/// fade (no per-particle color updates) so all particles can share two
/// pre-made HDR materials.
#[derive(Component)]
pub struct ThrusterParticle {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec2,
    pub initial_size: f32,
}

#[derive(Resource)]
pub struct ThrusterAssets {
    pub flame_mesh: Handle<Mesh>,
    pub main_material: Handle<ColorMaterial>,
    pub rcs_material: Handle<ColorMaterial>,
    pub main_core_material: Handle<ColorMaterial>,
    pub rcs_core_material: Handle<ColorMaterial>,
    pub particle_hot_material: Handle<ColorMaterial>,
    pub particle_ember_material: Handle<ColorMaterial>,
}

pub struct ThrusterPlugin;

impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            FixedUpdate,
            (
                attach_thrusters_to_player,
                update_thruster_intensity,
                emit_thruster_particles,
                update_thruster_particles,
            )
                .chain(),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let flame_mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let main_material = materials.add(ColorMaterial::from(linear_rgba(THRUSTER_FLAME_COLOR)));
    let rcs_material = materials.add(ColorMaterial::from(linear_rgba(THRUSTER_RCS_COLOR)));
    let main_core_material =
        materials.add(ColorMaterial::from(linear_rgba(THRUSTER_MAIN_CORE_COLOR)));
    let rcs_core_material =
        materials.add(ColorMaterial::from(linear_rgba(THRUSTER_RCS_CORE_COLOR)));
    let particle_hot_material =
        materials.add(ColorMaterial::from(linear_rgba(THRUSTER_PARTICLE_HOT_COLOR)));
    let particle_ember_material = materials.add(ColorMaterial::from(linear_rgba(
        THRUSTER_PARTICLE_EMBER_COLOR,
    )));
    commands.insert_resource(ThrusterAssets {
        flame_mesh,
        main_material,
        rcs_material,
        main_core_material,
        rcs_core_material,
        particle_hot_material,
        particle_ember_material,
    });
}

fn linear_rgba(c: [f32; 4]) -> Color {
    Color::linear_rgba(c[0], c[1], c[2], c[3])
}

fn update_thruster_intensity(
    keyboard: Res<ButtonInput<KeyCode>>,
    paused: Res<GamePaused>,
    dt: Res<GameTimeDelta>,
    mut q: Query<(&mut Thruster, &mut Transform)>,
) {
    let active = !paused.0;
    let f = active && keyboard.pressed(KeyCode::KeyW);
    let b = active && keyboard.pressed(KeyCode::KeyS);
    let a = active && keyboard.pressed(KeyCode::KeyA);
    let d = active && keyboard.pressed(KeyCode::KeyD);

    let alpha = (THRUSTER_INTENSITY_LERP_RATE * dt.0).clamp(0.0, 1.0);

    for (mut thruster, mut transform) in q.iter_mut() {
        let target = match thruster.direction {
            ThrusterDir::Forward => if f { 1.0 } else { 0.0 },
            ThrusterDir::Reverse => if b { REVERSE_SPEED_FRACTION } else { 0.0 },
            ThrusterDir::StrafeLeft => if a { 1.0 } else { 0.0 },
            ThrusterDir::StrafeRight => if d { 1.0 } else { 0.0 },
        };
        thruster.intensity += (target - thruster.intensity) * alpha;
        if target == 0.0 && thruster.intensity < THRUSTER_OFF_THRESHOLD {
            thruster.intensity = 0.0;
        }

        let length = thruster.max_length * thruster.intensity;
        let center = thruster.nozzle + thruster.exhaust * (length * 0.5);
        transform.translation.x = center.x;
        transform.translation.y = center.y;
        transform.scale.x = thruster.width;
        // Avoid an exact-zero scale (degenerate matrix).
        transform.scale.y = length.max(0.0001);
    }
}

/// Spawn exhaust particles from the forward thruster only — keeps the trail's
/// warm palette consistent. RCS bursts are short enough that the flame alone
/// communicates the action without needing particle feedback.
fn emit_thruster_particles(
    mut commands: Commands,
    paused: Res<GamePaused>,
    dt: Res<GameTimeDelta>,
    assets: Res<ThrusterAssets>,
    mut q: Query<(&mut Thruster, &GlobalTransform)>,
) {
    if paused.0 {
        return;
    }
    let mut rng = rand::thread_rng();

    for (mut thruster, gtf) in q.iter_mut() {
        if thruster.direction != ThrusterDir::Forward {
            continue;
        }
        if thruster.intensity < THRUSTER_EMIT_THRESHOLD {
            thruster.emit_timer = 0.0;
            continue;
        }
        thruster.emit_timer += dt.0;
        let interval = THRUSTER_PARTICLE_INTERVAL / thruster.intensity.max(0.1);
        if thruster.emit_timer < interval {
            continue;
        }
        thruster.emit_timer = 0.0;

        // The thruster entity sits at the flame midpoint; the far tip is half
        // the current length further along world-space exhaust direction.
        let length = thruster.max_length * thruster.intensity;
        let world_rot = gtf.compute_transform().rotation;
        let world_exhaust = (world_rot * Vec3::Y).truncate();
        let world_perp = Vec2::new(-world_exhaust.y, world_exhaust.x);
        let mut tip = gtf.translation();
        tip.x += world_exhaust.x * (length * 0.5);
        tip.y += world_exhaust.y * (length * 0.5);
        // Sit just behind the ship body in z so particles don't visually punch
        // through the hull on the frame they spawn.
        tip.z = -0.03;

        for _ in 0..THRUSTER_PARTICLES_PER_EMIT {
            let perp_offset = rng.gen_range(-0.5..0.5) * thruster.width;
            let perp_speed = rng.gen_range(-1.0..1.0) * THRUSTER_PARTICLE_SPREAD;
            let speed_var = rng.gen_range(0.7..1.2);
            let velocity = world_exhaust * (THRUSTER_PARTICLE_SPEED * speed_var)
                + world_perp * perp_speed;
            let mut pos = tip;
            pos.x += world_perp.x * perp_offset;
            pos.y += world_perp.y * perp_offset;

            let material = if rng.gen_bool(THRUSTER_PARTICLE_HOT_PROBABILITY) {
                assets.particle_hot_material.clone()
            } else {
                assets.particle_ember_material.clone()
            };
            let initial_size = THRUSTER_PARTICLE_INITIAL_SIZE * rng.gen_range(0.7..1.2);

            commands.spawn((
                Mesh2d(assets.flame_mesh.clone()),
                MeshMaterial2d(material),
                Transform {
                    translation: pos,
                    scale: Vec3::new(initial_size, initial_size, 1.0),
                    rotation: Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::TAU)),
                },
                ThrusterParticle {
                    lifetime: 0.0,
                    max_lifetime: THRUSTER_PARTICLE_LIFETIME,
                    velocity,
                    initial_size,
                },
            ));
        }
    }
}

/// Advance each particle's position and shrink its scale toward zero. Easing
/// holds size for most of the lifetime then drops fast at the end so the
/// plume reads as a soft fade rather than uniform shrinking.
fn update_thruster_particles(
    mut commands: Commands,
    paused: Res<GamePaused>,
    dt: Res<GameTimeDelta>,
    mut q: Query<(Entity, &mut ThrusterParticle, &mut Transform)>,
) {
    if paused.0 {
        return;
    }
    for (e, mut p, mut tf) in q.iter_mut() {
        p.lifetime += dt.0;
        if p.lifetime >= p.max_lifetime {
            commands.entity(e).despawn();
            continue;
        }
        tf.translation.x += p.velocity.x * dt.0;
        tf.translation.y += p.velocity.y * dt.0;
        let t = p.lifetime / p.max_lifetime;
        let size = p.initial_size * (1.0 - t).sqrt();
        tf.scale.x = size;
        tf.scale.y = size;
    }
}

/// When a Player ship appears, attach the four thruster children. Done in a
/// system rather than in the spawn template so the thruster module owns its
/// own assets instead of coupling them into PlayerShipResources.
fn attach_thrusters_to_player(
    mut commands: Commands,
    assets: Res<ThrusterAssets>,
    new_players: Query<Entity, Added<Player>>,
) {
    for player in new_players.iter() {
        let forward = spawn_thruster(
            &mut commands,
            &assets,
            ThrusterDir::Forward,
            Vec2::from(THRUSTER_FORWARD_OFFSET),
            Vec2::new(0.0, -1.0),
            PI,
            THRUSTER_MAIN_LENGTH,
            THRUSTER_MAIN_WIDTH,
            true,
        );
        let reverse = spawn_thruster(
            &mut commands,
            &assets,
            ThrusterDir::Reverse,
            Vec2::from(THRUSTER_REVERSE_OFFSET),
            Vec2::new(0.0, 1.0),
            0.0,
            THRUSTER_RCS_LENGTH,
            THRUSTER_RCS_WIDTH,
            false,
        );
        let strafe_left = spawn_thruster(
            &mut commands,
            &assets,
            ThrusterDir::StrafeLeft,
            Vec2::from(THRUSTER_STRAFE_LEFT_OFFSET),
            Vec2::new(1.0, 0.0),
            -FRAC_PI_2,
            THRUSTER_RCS_LENGTH,
            THRUSTER_RCS_WIDTH,
            false,
        );
        let strafe_right = spawn_thruster(
            &mut commands,
            &assets,
            ThrusterDir::StrafeRight,
            Vec2::from(THRUSTER_STRAFE_RIGHT_OFFSET),
            Vec2::new(-1.0, 0.0),
            FRAC_PI_2,
            THRUSTER_RCS_LENGTH,
            THRUSTER_RCS_WIDTH,
            false,
        );
        commands
            .entity(player)
            .add_children(&[forward, reverse, strafe_left, strafe_right]);
    }
}

/// Spawn an outer flame entity with a brighter inner-core child mesh. The
/// core inherits parent rotation/translation/scale via standard transform
/// propagation and uses a fixed local scale (CORE_*_FACTOR) so its size
/// tracks the outer flame automatically as intensity changes.
fn spawn_thruster(
    commands: &mut Commands,
    assets: &ThrusterAssets,
    direction: ThrusterDir,
    nozzle: Vec2,
    exhaust: Vec2,
    rotation_z: f32,
    max_length: f32,
    width: f32,
    main: bool,
) -> Entity {
    let (outer_mat, core_mat) = if main {
        (
            assets.main_material.clone(),
            assets.main_core_material.clone(),
        )
    } else {
        (
            assets.rcs_material.clone(),
            assets.rcs_core_material.clone(),
        )
    };

    let outer = commands
        .spawn((
            Mesh2d(assets.flame_mesh.clone()),
            MeshMaterial2d(outer_mat),
            Transform {
                translation: Vec3::new(nozzle.x, nozzle.y, -0.05),
                rotation: Quat::from_rotation_z(rotation_z),
                scale: Vec3::new(width, 0.0001, 1.0),
            },
            Thruster {
                direction,
                nozzle,
                exhaust,
                max_length,
                width,
                intensity: 0.0,
                emit_timer: 0.0,
            },
        ))
        .id();

    // Core: child of the outer. Local scale multiplies the outer's animated
    // (width, length) scale so the core stays a fixed fraction of the flame.
    // Local z = small positive so the core renders above the outer.
    let core = commands
        .spawn((
            Mesh2d(assets.flame_mesh.clone()),
            MeshMaterial2d(core_mat),
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.01),
                scale: Vec3::new(THRUSTER_CORE_WIDTH_FACTOR, THRUSTER_CORE_LENGTH_FACTOR, 1.0),
                ..default()
            },
        ))
        .id();
    commands.entity(outer).add_children(&[core]);
    outer
}
