//! Camera follow system: smooth lerp tracking the player ship, plus zoom.

use bevy::prelude::*;

use super::constants::controls::{CAMERA_FOLLOW_LERP, CAMERA_ZOOM_LERP};
use super::{CameraZoom, Player};

/// Smoothly moves the camera toward the player ship position and lerps the
/// orthographic projection scale toward the requested zoom target.
pub fn camera_follow_player(
    zoom: Res<CameraZoom>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((mut cam_transform, mut projection)) = camera_query.single_mut() else {
        return;
    };

    let target = player_transform.translation().truncate();
    let current = cam_transform.translation.truncate();
    let smoothed = current.lerp(target, CAMERA_FOLLOW_LERP);

    cam_transform.translation.x = smoothed.x;
    cam_transform.translation.y = smoothed.y;

    if let Projection::Orthographic(ortho) = projection.as_mut() {
        ortho.scale += (zoom.target_scale - ortho.scale) * CAMERA_ZOOM_LERP;
    }
}
