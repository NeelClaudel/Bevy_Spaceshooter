//! Parallax starfield background. Stars wrap around the camera so a small
//! finite set of entities looks infinite.

use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const STAR_TILE_SIZE: f32 = 2400.0;
const STAR_SEED: u64 = 42;

/// (parallax_factor, count, radius, hdr_brightness, z)
/// parallax 1.0 = world-fixed, 0.0 = camera-locked.
const STAR_LAYERS: [(f32, usize, f32, f32, f32); 3] = [
    (0.15, 120, 1.0, 1.5, -120.0),
    (0.40, 90,  1.5, 2.5, -110.0),
    (0.85, 60,  2.5, 4.0, -100.0),
];

#[derive(Component)]
pub struct Star {
    pub parallax: f32,
    pub base: Vec2,
}

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_starfield);
        app.add_systems(Update, update_star_positions);
    }
}

fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = StdRng::seed_from_u64(STAR_SEED);
    let half = STAR_TILE_SIZE * 0.5;

    for (parallax, count, radius, brightness, z) in STAR_LAYERS {
        let mesh = meshes.add(Circle::new(radius));
        let material = materials.add(ColorMaterial::from(Color::linear_rgba(
            brightness,
            brightness,
            brightness,
            1.0,
        )));

        for _ in 0..count {
            let base = Vec2::new(rng.gen_range(-half..half), rng.gen_range(-half..half));

            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_translation(base.extend(z)),
                Star { parallax, base },
            ));
        }
    }
}

fn update_star_positions(
    camera_q: Query<&Transform, (With<Camera2d>, Without<Star>)>,
    mut stars: Query<(&Star, &mut Transform)>,
) {
    let Ok(cam) = camera_q.single() else {
        return;
    };
    let cam_pos = cam.translation.truncate();
    let half = STAR_TILE_SIZE * 0.5;

    for (star, mut transform) in stars.iter_mut() {
        let mut offset = star.base - cam_pos * star.parallax;
        offset.x = (offset.x + half).rem_euclid(STAR_TILE_SIZE) - half;
        offset.y = (offset.y + half).rem_euclid(STAR_TILE_SIZE) - half;
        let world = cam_pos + offset;
        transform.translation.x = world.x;
        transform.translation.y = world.y;
    }
}
