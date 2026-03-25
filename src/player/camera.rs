//! Camera follow system: smooth lerp tracking the player ship.

use bevy::prelude::*;

use super::constants::controls::CAMERA_FOLLOW_LERP;
use super::Player;

/// Smoothly moves the camera toward the player ship position.
pub fn camera_follow_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut cam_transform) = camera_query.get_single_mut() else {
        return;
    };

    let target = player_transform.translation().truncate();
    let current = cam_transform.translation.truncate();
    let smoothed = current.lerp(target, CAMERA_FOLLOW_LERP);

    cam_transform.translation.x = smoothed.x;
    cam_transform.translation.y = smoothed.y;
}
