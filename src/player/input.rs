//! Player input systems: movement, targeting, pause, speed, energy.

use bevy::prelude::*;

use crate::game::GameSpeed;
use crate::movement::{MaxSpeed, MaxTurnSpeed, Speed, TurnSpeed};

use super::constants::controls::*;
use super::{AutoTarget, GamePaused, HoldFire, Player, PlayerTarget, ShipReactor, SystemPower};

/// WASD movement: W=forward, S=reverse, A=turn left, D=turn right.
pub fn player_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    paused: Res<GamePaused>,
    mut query: Query<
        (&mut TurnSpeed, &MaxTurnSpeed, &mut Speed, &MaxSpeed),
        With<Player>,
    >,
) {
    if paused.0 {
        return;
    }

    for (mut turn_speed, max_turn, mut speed, max_speed) in query.iter_mut() {
        // Turning
        let mut turn = 0.0;
        if keyboard.pressed(KeyCode::KeyA) {
            turn += max_turn.radians_per_second;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            turn -= max_turn.radians_per_second;
        }
        turn_speed.radians_per_second = turn;

        // Thrust
        if keyboard.pressed(KeyCode::KeyW) {
            speed.0 = max_speed.0;
        } else if keyboard.pressed(KeyCode::KeyS) {
            speed.0 = -max_speed.0 * REVERSE_SPEED_FRACTION;
        } else {
            speed.0 *= DECELERATION_FACTOR;
            // Clamp near-zero to zero
            if speed.0.abs() < 0.1 {
                speed.0 = 0.0;
            }
        }
    }
}

/// Left-click to select a target. The closest targetable entity under the cursor is picked.
pub fn player_target_input(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_target: ResMut<PlayerTarget>,
    mut player_query: Query<&mut crate::combat::Target, With<Player>>,
    targetable: Query<
        (Entity, &GlobalTransform, &crate::combat::projectile::CircularHitBox),
        (With<crate::combat::Team>, Without<Player>),
    >,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Some(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        return;
    };

    // Find closest entity to click
    let mut best: Option<(Entity, f32)> = None;
    for (entity, transform, hitbox) in targetable.iter() {
        let pos = transform.translation().truncate();
        let dist = pos.distance(world_pos);
        let click_radius = hitbox.radius.max(MIN_CLICK_TARGET_RADIUS);
        if dist < click_radius {
            if best.is_none() || dist < best.unwrap().1 {
                best = Some((entity, dist));
            }
        }
    }

    player_target.0 = best.map(|(e, _)| e);

    for mut target in player_query.iter_mut() {
        target.0 = player_target.0;
    }
}

/// Space bar toggles pause. During pause, dt is forced to 0.
pub fn player_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut paused: ResMut<GamePaused>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        paused.0 = !paused.0;
    }
}

/// +/- keys cycle through game speed steps.
pub fn player_speed_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed: ResMut<GameSpeed>,
    paused: Res<GamePaused>,
) {
    if paused.0 {
        return;
    }

    let current_idx = GAME_SPEED_STEPS
        .iter()
        .position(|&s| (s - speed.0).abs() < 0.01)
        .unwrap_or(2);

    if keyboard.just_pressed(KeyCode::Equal) {
        let new_idx = (current_idx + 1).min(GAME_SPEED_STEPS.len() - 1);
        speed.0 = GAME_SPEED_STEPS[new_idx];
    }
    if keyboard.just_pressed(KeyCode::Minus) {
        let new_idx = current_idx.saturating_sub(1);
        speed.0 = GAME_SPEED_STEPS[new_idx];
    }
}

/// 1/2/3 = add power to system, Shift+1/2/3 = remove power from system.
pub fn player_energy_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut power: ResMut<SystemPower>,
    reactor_q: Query<&ShipReactor, With<Player>>,
) {
    let Ok(reactor) = reactor_q.get_single() else {
        return;
    };

    let total_used = power.shields.current + power.weapons.current + power.engines.current;
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    if keyboard.just_pressed(KeyCode::Digit1) {
        if shift {
            if power.shields.current > 0 {
                power.shields.current -= 1;
            }
        } else if total_used < reactor.max_power && power.shields.current < power.shields.max_level
        {
            power.shields.current += 1;
        }
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        if shift {
            if power.weapons.current > 0 {
                power.weapons.current -= 1;
            }
        } else if total_used < reactor.max_power && power.weapons.current < power.weapons.max_level
        {
            power.weapons.current += 1;
        }
    }

    if keyboard.just_pressed(KeyCode::Digit3) {
        if shift {
            if power.engines.current > 0 {
                power.engines.current -= 1;
            }
        } else if total_used < reactor.max_power && power.engines.current < power.engines.max_level
        {
            power.engines.current += 1;
        }
    }
}

/// F key toggles hold fire.
pub fn player_hold_fire_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hold_fire: ResMut<HoldFire>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        hold_fire.0 = !hold_fire.0;
    }
}

/// T key toggles auto-targeting.
pub fn player_auto_target_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut auto_target: ResMut<AutoTarget>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        auto_target.enabled = !auto_target.enabled;
        // Reset timer so it scans immediately when re-enabled
        auto_target.retarget_timer = 0.0;
    }
}

/// Automatically finds and targets the closest enemy within range.
/// Runs periodically based on AUTO_TARGET_RETARGET_INTERVAL.
/// Only active when AutoTarget is enabled and no manual target is alive.
pub fn auto_target_nearest_enemy(
    time: Res<Time>,
    mut auto_target: ResMut<AutoTarget>,
    mut player_target: ResMut<PlayerTarget>,
    player_query: Query<(&GlobalTransform, &crate::combat::Team), With<Player>>,
    mut target_query: Query<&mut crate::combat::Target, With<Player>>,
    enemies: Query<
        (Entity, &GlobalTransform, &crate::combat::Team, &crate::combat::mortal::Health),
        Without<Player>,
    >,
    // Check if current target is still alive
    alive_check: Query<&crate::combat::mortal::Health>,
) {
    if !auto_target.enabled {
        return;
    }

    // Tick the retarget timer (use real time, works even during pause for responsiveness)
    auto_target.retarget_timer -= time.delta_seconds();

    // If we have a valid living target, don't re-scan until timer expires
    if let Some(current) = player_target.0 {
        if alive_check.get(current).is_ok_and(|h| h.0 > 0.0) {
            if auto_target.retarget_timer > 0.0 {
                return;
            }
        }
    }

    // Time to scan
    auto_target.retarget_timer = AUTO_TARGET_RETARGET_INTERVAL;

    let Ok((player_transform, player_team)) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation();

    // Find closest living enemy within range
    let mut best: Option<(Entity, f32)> = None;
    for (entity, transform, team, health) in enemies.iter() {
        // Skip allies
        if team.0 == player_team.0 {
            continue;
        }
        // Skip dead
        if health.0 <= 0.0 {
            continue;
        }
        let dist = player_pos.distance(transform.translation());
        if dist > AUTO_TARGET_RANGE {
            continue;
        }
        if best.is_none() || dist < best.unwrap().1 {
            best = Some((entity, dist));
        }
    }

    let new_target = best.map(|(e, _)| e);

    // Only update if target changed
    if new_target != player_target.0 {
        player_target.0 = new_target;
        for mut target in target_query.iter_mut() {
            target.0 = new_target;
        }
    }
}
