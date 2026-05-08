//! Gatling: ballistic high-rate-of-fire weapon.
//!
//! Spawns small fast-moving bullet entities that travel in a straight line and
//! deal damage on impact. The bullet visual is a thin HDR-yellow rectangle
//! oriented along its velocity. Collision and damage application live in
//! `combat::ballistic`.

use bevy::prelude::*;
use bevy::sprite_render::{ColorMaterial, MeshMaterial2d};

use crate::combat::{
    attack::Attack,
    ballistic::BallisticProjectile,
    damage::Damage,
    effects::Instigator,
    lifetime::Lifetime,
    Team,
};
use crate::fx::{animated::AnimatedEffects, HitEffect};

/// Preloaded mesh + material for gatling bullets.
#[derive(Resource)]
pub struct GatlingResources {
    bullet_mesh: Handle<Mesh>,
    bullet_material: Handle<ColorMaterial>,
}

/// Spawns a single gatling bullet at `origin` moving at `velocity` (world units/sec).
/// `lifetime_seconds` should be tuned so the bullet expires near max range.
pub fn spawn_gatling_bullet(
    commands: &mut Commands,
    resources: &GatlingResources,
    origin: Vec3,
    velocity: Vec2,
    damage: f32,
    accuracy: f32,
    team: Team,
    instigator: Entity,
    lifetime_seconds: f32,
) {
    // Align the rectangle's long (Y) axis with the velocity direction.
    let angle = velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2;
    let transform = Transform {
        translation: origin,
        rotation: Quat::from_rotation_z(angle),
        ..default()
    };

    commands.spawn((
        BallisticProjectile { velocity },
        Damage::new(damage),
        Attack::new(accuracy),
        team,
        HitEffect {
            effect: AnimatedEffects::TinyPlusExplosion,
        },
        Instigator(instigator),
        Lifetime {
            seconds_remaining: lifetime_seconds,
        },
        Mesh2d(resources.bullet_mesh.clone()),
        MeshMaterial2d(resources.bullet_material.clone()),
        transform,
        GlobalTransform::default(),
    ));
}

pub struct GatlingTemplatePlugin;

impl GatlingTemplatePlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let bullet_mesh = meshes.add(Mesh::from(Rectangle::new(1.5, 6.0)));
        // HDR yellow so bloom picks it up.
        let bullet_material = materials.add(ColorMaterial::from(Color::linear_rgba(
            5.0, 3.5, 0.6, 1.0,
        )));
        commands.insert_resource(GatlingResources {
            bullet_mesh,
            bullet_material,
        });
    }
}

impl Plugin for GatlingTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, GatlingTemplatePlugin::setup);
    }
}
