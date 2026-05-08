//! Visual indicators: locked-target ring + cursor-hover ring (red/green by team).

use bevy::prelude::*;
use bevy::sprite_render::{ColorMaterial, MeshMaterial2d};

use crate::combat::projectile::CircularHitBox;
use crate::combat::Team;

use super::constants::controls::MIN_CLICK_TARGET_RADIUS;
use super::{Player, PlayerTarget};

const LOCK_RADIUS_FACTOR: f32 = 1.5;
const HOVER_RADIUS_FACTOR: f32 = 1.25;
const RING_INNER: f32 = 0.90;
const RING_OUTER: f32 = 1.00;
const INDICATOR_Z: f32 = 5.0;
const MIN_INDICATOR_RADIUS: f32 = 16.0;

// HDR colors so bloom picks them up.
const LOCK_COLOR: [f32; 4] = [3.5, 2.8, 0.4, 1.0];
const ENEMY_COLOR: [f32; 4] = [3.5, 0.3, 0.3, 0.85];
const FRIENDLY_COLOR: [f32; 4] = [0.3, 3.5, 0.3, 0.85];

#[derive(Component)]
pub struct LockIndicator;

#[derive(Component)]
pub struct HoverIndicator;

/// The entity currently under the mouse cursor (if any), excluding the player.
#[derive(Resource, Default)]
pub struct HoveredEntity(pub Option<Entity>);

pub struct IndicatorsPlugin;

impl Plugin for IndicatorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredEntity>();
        app.add_systems(Startup, spawn_indicators);
        app.add_systems(
            Update,
            (
                update_hovered_entity,
                update_lock_indicator,
                update_hover_indicator,
            ),
        );
    }
}

fn spawn_indicators(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ring_mesh = meshes.add(Annulus::new(RING_INNER, RING_OUTER));
    let [r, g, b, a] = LOCK_COLOR;
    let lock_mat = materials.add(ColorMaterial::from(Color::linear_rgba(r, g, b, a)));
    let [r, g, b, a] = ENEMY_COLOR;
    let hover_mat = materials.add(ColorMaterial::from(Color::linear_rgba(r, g, b, a)));

    commands.spawn((
        Mesh2d(ring_mesh.clone()),
        MeshMaterial2d(lock_mat),
        Transform::from_translation(Vec3::new(0.0, 0.0, INDICATOR_Z)),
        Visibility::Hidden,
        LockIndicator,
    ));

    commands.spawn((
        Mesh2d(ring_mesh),
        MeshMaterial2d(hover_mat),
        Transform::from_translation(Vec3::new(0.0, 0.0, INDICATOR_Z)),
        Visibility::Hidden,
        HoverIndicator,
    ));
}

fn update_hovered_entity(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    targetable: Query<
        (Entity, &GlobalTransform, &CircularHitBox),
        (With<Team>, Without<Player>),
    >,
    mut hovered: ResMut<HoveredEntity>,
) {
    let Ok(window) = windows.single() else {
        hovered.0 = None;
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        hovered.0 = None;
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        hovered.0 = None;
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        hovered.0 = None;
        return;
    };

    let mut best: Option<(Entity, f32)> = None;
    for (entity, transform, hitbox) in targetable.iter() {
        let pos = transform.translation().truncate();
        let dist = pos.distance(world_pos);
        let pick_radius = hitbox.radius.max(MIN_CLICK_TARGET_RADIUS);
        if dist < pick_radius && (best.is_none() || dist < best.unwrap().1) {
            best = Some((entity, dist));
        }
    }
    hovered.0 = best.map(|(e, _)| e);
}

fn update_lock_indicator(
    target: Res<PlayerTarget>,
    targets: Query<(&GlobalTransform, &CircularHitBox)>,
    mut indicator: Query<(&mut Transform, &mut Visibility), With<LockIndicator>>,
) {
    let Ok((mut transform, mut visibility)) = indicator.single_mut() else {
        return;
    };
    let Some(target_entity) = target.0 else {
        *visibility = Visibility::Hidden;
        return;
    };
    let Ok((target_transform, hitbox)) = targets.get(target_entity) else {
        *visibility = Visibility::Hidden;
        return;
    };

    let scale = (hitbox.radius * LOCK_RADIUS_FACTOR).max(MIN_INDICATOR_RADIUS);
    let pos = target_transform.translation();
    transform.translation.x = pos.x;
    transform.translation.y = pos.y;
    transform.translation.z = INDICATOR_Z;
    transform.scale = Vec3::new(scale, scale, 1.0);
    *visibility = Visibility::Inherited;
}

fn update_hover_indicator(
    hovered: Res<HoveredEntity>,
    target: Res<PlayerTarget>,
    player_team_q: Query<&Team, With<Player>>,
    targets: Query<(&GlobalTransform, &CircularHitBox, &Team)>,
    mut indicator: Query<
        (&mut Transform, &mut Visibility, &MeshMaterial2d<ColorMaterial>),
        With<HoverIndicator>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((mut transform, mut visibility, material_handle)) = indicator.single_mut() else {
        return;
    };

    let Some(hovered_entity) = hovered.0 else {
        *visibility = Visibility::Hidden;
        return;
    };
    // Lock ring already shows on this entity — don't double up.
    if Some(hovered_entity) == target.0 {
        *visibility = Visibility::Hidden;
        return;
    }
    let Ok((hovered_transform, hitbox, team)) = targets.get(hovered_entity) else {
        *visibility = Visibility::Hidden;
        return;
    };
    let Ok(player_team) = player_team_q.single() else {
        *visibility = Visibility::Hidden;
        return;
    };

    let [r, g, b, a] = if team.0 == player_team.0 {
        FRIENDLY_COLOR
    } else {
        ENEMY_COLOR
    };
    if let Some(mat) = materials.get_mut(&material_handle.0) {
        mat.color = Color::linear_rgba(r, g, b, a);
    }

    let scale = (hitbox.radius * HOVER_RADIUS_FACTOR).max(MIN_INDICATOR_RADIUS);
    let pos = hovered_transform.translation();
    transform.translation.x = pos.x;
    transform.translation.y = pos.y;
    transform.translation.z = INDICATOR_Z;
    transform.scale = Vec3::new(scale, scale, 1.0);
    *visibility = Visibility::Inherited;
}
