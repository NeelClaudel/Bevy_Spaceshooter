//! Player input systems: movement, targeting, pause, speed, energy, firing.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::ecs::message::MessageReader;

use crate::combat::attack::{Attack, AttackResult};
use crate::combat::effects::{Effect, EffectLocation, Effectiveness, Effector, Instigator, SourceTransform};
use crate::combat::mortal::Dieing;
use crate::combat::projectile::CircularHitBox;
use crate::combat::tools::{Cooldown, TargettedTool};
use crate::combat::{Target, Team};
use crate::game::{GameSpeed, GameTimeDelta};
use crate::math_util;
use crate::movement::{Heading, MaxSpeed, MaxTurnSpeed, Speed, TurnSpeed, Velocity};

use super::constants::controls::*;
use super::indicators::HoveredEntity;
use super::{
    Ammunition, AutoFire, BallisticConfig, CameraZoom, CursorWorldPos, GamePaused, Player,
    PlayerFireInput, PlayerLocalVelocity, PlayerTarget, PlayerWeapon, PlayerWeaponGroup,
    ShipReactor, SystemPower,
};
use crate::templates::weapons::gatling::{spawn_gatling_bullet, GatlingResources};
use rand::Rng;

/// Twin-stick movement: W=thrust forward, S=thrust back, A=strafe left,
/// D=strafe right (all ship-relative). Rotation is handled by
/// `player_aim_at_cursor`. Velocity persists in `PlayerLocalVelocity` so it
/// decays smoothly when no key is held.
pub fn player_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    paused: Res<GamePaused>,
    mut query: Query<
        (
            &Transform,
            &MaxSpeed,
            &mut PlayerLocalVelocity,
            &mut Speed,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    if paused.0 {
        return;
    }

    for (transform, max_speed, mut local_vel, mut speed, mut velocity) in query.iter_mut() {
        let mut input = Vec2::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            input.y = 1.0;
        } else if keyboard.pressed(KeyCode::KeyS) {
            input.y = -REVERSE_SPEED_FRACTION;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            input.x = -1.0;
        } else if keyboard.pressed(KeyCode::KeyD) {
            input.x = 1.0;
        }
        // Cap diagonal magnitude so W+D isn't √2× faster than W alone.
        if input.length_squared() > 1.0 {
            input = input.normalize();
        }

        if input != Vec2::ZERO {
            local_vel.0 = input * max_speed.0;
        } else {
            local_vel.0 *= DECELERATION_FACTOR;
            if local_vel.0.length() < 0.1 {
                local_vel.0 = Vec2::ZERO;
            }
        }

        // Convert ship-local velocity (x=right, y=forward) to world space.
        let forward = (*transform.local_y()).truncate();
        let right = (*transform.local_x()).truncate();
        let world_vel = forward * local_vel.0.y + right * local_vel.0.x;
        velocity.0 = world_vel.extend(0.0);
        // Keep Speed.0 as the forward-axis scalar so evasion (which scales
        // with forward speed) behaves the same as before the rework.
        speed.0 = local_vel.0.y;
    }
}

/// Smoothly rotates the ship to face the world-space cursor. Turn rate is
/// capped by `MaxTurnSpeed` (which engine power scales), so the ship feels
/// heavier with low engine power and snappier with high.
pub fn player_aim_at_cursor(
    paused: Res<GamePaused>,
    cursor: Res<CursorWorldPos>,
    dt: Res<GameTimeDelta>,
    mut query: Query<(&Transform, &Heading, &MaxTurnSpeed, &mut TurnSpeed), With<Player>>,
) {
    if paused.0 {
        return;
    }
    let Some(cursor_pos) = cursor.0 else {
        for (_, _, _, mut ts) in query.iter_mut() {
            ts.radians_per_second = 0.0;
        }
        return;
    };
    for (transform, heading, max_turn, mut turn_speed) in query.iter_mut() {
        let to_cursor = cursor_pos - transform.translation.truncate();
        if to_cursor.length() < AIM_DEADZONE_RADIUS {
            turn_speed.radians_per_second = 0.0;
            continue;
        }
        let target = math_util::get_heading_to_point(to_cursor.extend(0.0));
        let delta = math_util::get_angle_difference(target, heading.radians);
        // Pick the rate that lands on the target this tick if possible,
        // otherwise saturate to MaxTurnSpeed.
        let desired = if dt.0 > 0.0 { delta / dt.0 } else { 0.0 };
        let max = max_turn.radians_per_second;
        turn_speed.radians_per_second = desired.clamp(-max, max);
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

    if keyboard.just_pressed(KeyCode::NumpadAdd) {
        let new_idx = (current_idx + 1).min(GAME_SPEED_STEPS.len() - 1);
        speed.0 = GAME_SPEED_STEPS[new_idx];
    }
    if keyboard.just_pressed(KeyCode::NumpadSubtract) {
        let new_idx = current_idx.saturating_sub(1);
        speed.0 = GAME_SPEED_STEPS[new_idx];
    }
}

/// Mouse wheel adjusts camera zoom target. Each notch multiplies the target
/// scale by CAMERA_ZOOM_STEP (up = zoom in / smaller scale, down = zoom out).
pub fn player_zoom_input(
    mut wheel: MessageReader<MouseWheel>,
    mut zoom: ResMut<CameraZoom>,
) {
    let mut delta = 0.0_f32;
    for ev in wheel.read() {
        delta += ev.y;
    }
    if delta == 0.0 {
        return;
    }

    let factor = CAMERA_ZOOM_STEP.powf(-delta);
    zoom.target_scale = (zoom.target_scale * factor).clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
}

/// 1/2/3 = add power to system, Shift+1/2/3 = remove power from system.
pub fn player_energy_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut power: ResMut<SystemPower>,
    reactor_q: Query<&ShipReactor, With<Player>>,
) {
    let Ok(reactor) = reactor_q.single() else {
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

/// F key toggles auto-fire.
pub fn player_auto_fire_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut auto_fire: ResMut<AutoFire>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        auto_fire.0 = !auto_fire.0;
    }
}

/// T key locks the currently-hovered entity as the player target.
/// If nothing is hovered, clears the lock.
pub fn player_lock_target_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    hovered: Res<HoveredEntity>,
    mut player_target: ResMut<PlayerTarget>,
    mut player_query: Query<&mut Target, With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyT) {
        return;
    }

    player_target.0 = hovered.0;
    for mut target in player_query.iter_mut() {
        target.0 = hovered.0;
    }
}

/// Clears PlayerTarget (and the player's Target component) when the locked
/// entity no longer exists OR has entered the dying state. Without the
/// despawn check, a stale lock causes targeted firing logic to skip every
/// tick forever; without the dying check, the lock ring keeps tracking a
/// wreck while it explodes — visually confusing.
pub fn clear_dead_target_lock(
    mut player_target: ResMut<PlayerTarget>,
    transforms: Query<&GlobalTransform>,
    dieing: Query<(), With<Dieing>>,
    mut player_query: Query<&mut Target, With<Player>>,
) {
    let Some(target) = player_target.0 else {
        return;
    };
    if transforms.get(target).is_err() || dieing.get(target).is_ok() {
        player_target.0 = None;
        for mut t in player_query.iter_mut() {
            t.0 = None;
        }
    }
}

/// Samples mouse buttons into the PlayerFireInput resource each frame.
/// Left = primary weapons, Right = missiles. Both can be held to keep firing.
pub fn player_mouse_fire_input(
    mouse: Res<ButtonInput<MouseButton>>,
    paused: Res<GamePaused>,
    mut fire_input: ResMut<PlayerFireInput>,
) {
    if paused.0 {
        fire_input.primary_held = false;
        fire_input.missile_held = false;
        return;
    }
    fire_input.primary_held = mouse.pressed(MouseButton::Left);
    fire_input.missile_held = mouse.pressed(MouseButton::Right);
}

/// Tracks the cursor's world-space position each frame from window + camera.
pub fn update_cursor_world_pos(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<CursorWorldPos>,
) {
    let Ok(window) = windows.single() else {
        cursor.0 = None;
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        cursor.0 = None;
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        cursor.0 = None;
        return;
    };
    cursor.0 = camera.viewport_to_world_2d(cam_transform, cursor_pos).ok();
}

/// Missile firing: requires a locked target (PlayerTarget). Right-click holds the
/// trigger; auto-fire (F) also fires when no manual input. Missile launchers
/// continue to use the standard Effector path (apply_effects spawns the rocket).
pub fn player_fire_missiles(
    fire_input: Res<PlayerFireInput>,
    auto_fire: Res<AutoFire>,
    player_target: Res<PlayerTarget>,
    paused: Res<GamePaused>,
    pos_query: Query<&GlobalTransform>,
    mut turrets: Query<(
        &mut Cooldown,
        &mut TargettedTool,
        &Target,
        &GlobalTransform,
        &PlayerWeapon,
        Option<&mut Ammunition>,
    )>,
) {
    if paused.0 {
        return;
    }

    for (mut cooldown, mut tool, target, transform, weapon, ammo) in turrets.iter_mut() {
        if !matches!(weapon.group, PlayerWeaponGroup::Missile) {
            continue;
        }
        if !tool.armed || !cooldown.is_ready() {
            continue;
        }

        let manual = fire_input.missile_held;
        let auto = auto_fire.0;
        if !manual && !auto {
            continue;
        }

        if player_target.0.is_none() {
            continue;
        }

        let Some(target_entity) = target.0 else {
            continue;
        };
        let Ok(target_transform) = pos_query.get(target_entity) else {
            continue;
        };

        let delta = target_transform.translation() - transform.translation();
        if delta.length_squared() > tool.range * tool.range {
            continue;
        }
        let projection = delta.normalize().dot(transform.up().normalize());
        if projection < (tool.cone / 2.0).cos() {
            continue;
        }

        if let Some(mut ammo) = ammo {
            if ammo.current == 0 {
                continue;
            }
            ammo.current -= 1;
        }

        tool.firing = true;
        cooldown.reset();
        if !manual && auto {
            cooldown.remaining = cooldown.duration * AUTO_FIRE_COOLDOWN_PENALTY;
        }
    }
}

/// Primary-weapon firing: each ready primary turret fires toward the world-space
/// cursor. Hitscan turrets (lasers/flak) draw a beam and resolve damage through
/// the regular Attack/Damage/Shield pipeline; ballistic turrets (gatling) spawn
/// a bullet entity that travels in a straight line and resolves on collision.
/// No cone gate for player primaries — fires 360° around the ship.
pub fn player_primary_fire(
    mut commands: Commands,
    fire_input: Res<PlayerFireInput>,
    auto_fire: Res<AutoFire>,
    paused: Res<GamePaused>,
    cursor: Res<CursorWorldPos>,
    gatling_res: Res<GatlingResources>,
    enemies: Query<(Entity, &GlobalTransform, &CircularHitBox, &Team)>,
    player_team_q: Query<&Team, With<Player>>,
    mut turrets: Query<(
        Entity,
        &mut Cooldown,
        &TargettedTool,
        Option<&Effector>,
        Option<&BallisticConfig>,
        &PlayerWeapon,
        &GlobalTransform,
    )>,
) {
    if paused.0 {
        return;
    }
    let manual = fire_input.primary_held;
    let auto = auto_fire.0;
    if !manual && !auto {
        return;
    }
    let Some(cursor_pos) = cursor.0 else { return; };
    let Ok(player_team) = player_team_q.single() else { return; };

    let mut rng = rand::thread_rng();

    for (turret_entity, mut cooldown, tool, effector, ballistic, weapon, gtf) in turrets.iter_mut()
    {
        if !matches!(weapon.group, PlayerWeaponGroup::Primary) {
            continue;
        }
        if !tool.armed || !cooldown.is_ready() {
            continue;
        }

        let turret_pos = gtf.translation();
        let aim2 = cursor_pos - turret_pos.truncate();
        if aim2.length_squared() < 0.001 {
            continue;
        }
        let dir2 = aim2.normalize();

        if let Some(ballistic) = ballistic {
            // Ballistic path: spawn a bullet with random aim jitter.
            let jitter: f32 = rng.gen_range(-ballistic.spread..ballistic.spread);
            let (sin, cos) = jitter.sin_cos();
            let aimed = Vec2::new(
                dir2.x * cos - dir2.y * sin,
                dir2.x * sin + dir2.y * cos,
            );
            let velocity = aimed * ballistic.bullet_speed;
            let lifetime = if ballistic.bullet_speed > 0.0 {
                tool.range / ballistic.bullet_speed
            } else {
                1.0
            };
            spawn_gatling_bullet(
                &mut commands,
                &gatling_res,
                turret_pos,
                velocity,
                ballistic.damage,
                ballistic.accuracy,
                *player_team,
                turret_entity,
                lifetime,
            );
        } else if let Some(effector) = effector {
            // Hitscan path: ray-vs-hitbox against enemies.
            let dir = dir2.extend(0.0);
            let mut hit: Option<(Entity, f32, Vec3)> = None;
            for (e, etf, hb, team) in enemies.iter() {
                if team.0 == player_team.0 {
                    continue;
                }
                let to_enemy = etf.translation() - turret_pos;
                let t = to_enemy.dot(dir);
                if t < 0.0 || t > tool.range {
                    continue;
                }
                let closest = turret_pos + dir * t;
                let dist_sq = (etf.translation() - closest).length_squared();
                if dist_sq > hb.radius * hb.radius {
                    continue;
                }
                if hit.map_or(true, |(_, prev_t, _)| t < prev_t) {
                    hit = Some((e, t, etf.translation()));
                }
            }

            let (target_entity, end_pos, result) = match hit {
                Some((e, _, pos)) => (e, pos, AttackResult::Hit),
                None => {
                    let end = turret_pos + dir * tool.range;
                    (turret_entity, end, AttackResult::Miss)
                }
            };

            let spawned = (effector.spawn_effect)(&mut commands);
            let mut e = commands.entity(spawned);
            e.insert((
                Target(Some(target_entity)),
                Instigator(turret_entity),
                SourceTransform(*gtf),
                Transform::from_translation(turret_pos),
                *gtf,
                Effect,
                Effectiveness::default(),
                EffectLocation(end_pos),
            ));
            if result == AttackResult::Miss {
                // Override the Hit result the factory inserts; pulse_laser_attack uses
                // accuracy 3.0 today and is the only hitscan factory in play.
                e.insert(Attack {
                    accuracy: 3.0,
                    result: AttackResult::Miss,
                });
            }
        } else {
            // Primary turret with neither Effector nor BallisticConfig — skip.
            continue;
        }

        cooldown.reset();
        if !manual && auto {
            cooldown.remaining = cooldown.duration * AUTO_FIRE_COOLDOWN_PENALTY;
        }
    }
}
