//! HUD overlay: health, shield, energy, target info, game speed.

use bevy::prelude::*;

use crate::combat::mortal::{Health, MaxHealth};
use crate::combat::shields::{MaxShieldHP, Shield};
use crate::game::GameSpeed;

use super::constants::hud::*;
use super::{AutoTarget, GamePaused, HoldFire, Player, PlayerTarget, SystemPower};

// ---------------------------------------------------------------------------
// Marker components for updatable HUD elements
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct ShieldBarFill;

#[derive(Component)]
pub struct EnergyBarShields;

#[derive(Component)]
pub struct EnergyBarWeapons;

#[derive(Component)]
pub struct EnergyBarEngines;

#[derive(Component)]
pub struct TargetInfoText;

#[derive(Component)]
pub struct GameSpeedText;

// ---------------------------------------------------------------------------
// Spawn HUD
// ---------------------------------------------------------------------------

fn color_from_arr(c: [f32; 4]) -> Color {
    Color::srgba(c[0], c[1], c[2], c[3])
}

fn bar_background(width: f32, height: f32) -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
        BackgroundColor(color_from_arr(BAR_BACKGROUND_COLOR)),
    )
}

fn bar_fill(color: [f32; 4], width: f32, height: f32) -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
        BackgroundColor(color_from_arr(color)),
    )
}

pub fn spawn_hud(mut commands: Commands) {
    // --- Bottom-left panel: health, shield, energy ---
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(HUD_MARGIN),
                bottom: Val::Px(HUD_MARGIN),
                padding: UiRect::all(Val::Px(HUD_PADDING)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
        ))
        .with_children(|parent| {
            // Health label + bar
            parent.spawn((
                Text::new("HP"),
                TextFont { font_size: HUD_FONT_SIZE, ..default() },
                TextColor(color_from_arr(HUD_LABEL_COLOR)),
            ));
            parent
                .spawn(bar_background(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(HEALTH_BAR_COLOR, HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT))
                        .insert(HealthBarFill);
                });

            // Shield label + bar
            parent.spawn((
                Text::new("SHIELD"),
                TextFont { font_size: HUD_FONT_SIZE, ..default() },
                TextColor(color_from_arr(HUD_LABEL_COLOR)),
            ));
            parent
                .spawn(bar_background(SHIELD_BAR_WIDTH, SHIELD_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(SHIELD_BAR_COLOR, SHIELD_BAR_WIDTH, SHIELD_BAR_HEIGHT))
                        .insert(ShieldBarFill);
                });

            // Energy: Shields
            parent.spawn((
                Text::new("1: Shields"),
                TextFont { font_size: HUD_FONT_SIZE, ..default() },
                TextColor(color_from_arr(ENERGY_SHIELD_COLOR)),
            ));
            parent
                .spawn(bar_background(ENERGY_BAR_WIDTH, ENERGY_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(
                        ENERGY_SHIELD_COLOR,
                        ENERGY_BAR_WIDTH,
                        ENERGY_BAR_HEIGHT,
                    ))
                    .insert(EnergyBarShields);
                });

            // Energy: Weapons
            parent.spawn((
                Text::new("2: Weapons"),
                TextFont { font_size: HUD_FONT_SIZE, ..default() },
                TextColor(color_from_arr(ENERGY_WEAPON_COLOR)),
            ));
            parent
                .spawn(bar_background(ENERGY_BAR_WIDTH, ENERGY_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(
                        ENERGY_WEAPON_COLOR,
                        ENERGY_BAR_WIDTH,
                        ENERGY_BAR_HEIGHT,
                    ))
                    .insert(EnergyBarWeapons);
                });

            // Energy: Engines
            parent.spawn((
                Text::new("3: Engines"),
                TextFont { font_size: HUD_FONT_SIZE, ..default() },
                TextColor(color_from_arr(ENERGY_ENGINE_COLOR)),
            ));
            parent
                .spawn(bar_background(ENERGY_BAR_WIDTH, ENERGY_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(
                        ENERGY_ENGINE_COLOR,
                        ENERGY_BAR_WIDTH,
                        ENERGY_BAR_HEIGHT,
                    ))
                    .insert(EnergyBarEngines);
                });
        });

    // --- Top-left: Game speed / pause indicator ---
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(HUD_MARGIN),
                top: Val::Px(HUD_MARGIN),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("Speed: x1.0"),
                    TextFont { font_size: HUD_FONT_SIZE, ..default() },
                    TextColor(color_from_arr(HUD_LABEL_COLOR)),
                ))
                .insert(GameSpeedText);
        });

    // --- Top-right: Target info ---
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(HUD_MARGIN),
                top: Val::Px(HUD_MARGIN),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("No target"),
                    TextFont { font_size: HUD_FONT_SIZE, ..default() },
                    TextColor(color_from_arr(TARGET_INDICATOR_COLOR)),
                ))
                .insert(TargetInfoText);
        });
}

// ---------------------------------------------------------------------------
// Update systems
// ---------------------------------------------------------------------------

pub fn update_health_bar(
    player_q: Query<(&Health, &MaxHealth), With<Player>>,
    mut bar_q: Query<&mut Node, With<HealthBarFill>>,
) {
    let Ok((health, max_health)) = player_q.single() else {
        return;
    };
    let Ok(mut node) = bar_q.single_mut() else {
        return;
    };
    let fraction = (health.0 / max_health.0).clamp(0.0, 1.0);
    node.width = Val::Px(HEALTH_BAR_WIDTH * fraction);
}

pub fn update_shield_bar(
    player_q: Query<(&Shield, &MaxShieldHP), With<Player>>,
    mut bar_q: Query<&mut Node, With<ShieldBarFill>>,
) {
    let Ok((shield, max_shield)) = player_q.single() else {
        return;
    };
    let Ok(mut node) = bar_q.single_mut() else {
        return;
    };
    let fraction = (shield.health / max_shield.0.max(1.0)).clamp(0.0, 1.0);
    node.width = Val::Px(SHIELD_BAR_WIDTH * fraction);
}

pub fn update_energy_bars(
    power: Res<SystemPower>,
    mut shields_q: Query<&mut Node, (With<EnergyBarShields>, Without<EnergyBarWeapons>, Without<EnergyBarEngines>)>,
    mut weapons_q: Query<&mut Node, (With<EnergyBarWeapons>, Without<EnergyBarShields>, Without<EnergyBarEngines>)>,
    mut engines_q: Query<&mut Node, (With<EnergyBarEngines>, Without<EnergyBarShields>, Without<EnergyBarWeapons>)>,
) {
    if let Ok(mut node) = shields_q.single_mut() {
        let frac = power.shields.current as f32 / power.shields.max_level.max(1) as f32;
        node.width = Val::Px(ENERGY_BAR_WIDTH * frac);
    }
    if let Ok(mut node) = weapons_q.single_mut() {
        let frac = power.weapons.current as f32 / power.weapons.max_level.max(1) as f32;
        node.width = Val::Px(ENERGY_BAR_WIDTH * frac);
    }
    if let Ok(mut node) = engines_q.single_mut() {
        let frac = power.engines.current as f32 / power.engines.max_level.max(1) as f32;
        node.width = Val::Px(ENERGY_BAR_WIDTH * frac);
    }
}

pub fn update_target_info(
    player_target: Res<PlayerTarget>,
    player_q: Query<&GlobalTransform, With<Player>>,
    target_q: Query<(&Health, &MaxHealth, &GlobalTransform)>,
    mut text_q: Query<&mut Text, With<TargetInfoText>>,
) {
    let Ok(mut text) = text_q.single_mut() else {
        return;
    };

    let Some(target_entity) = player_target.0 else {
        **text = "No target".to_string();
        return;
    };

    let Ok((health, max_health, target_transform)) = target_q.get(target_entity) else {
        **text = "Target lost".to_string();
        return;
    };

    let distance = if let Ok(player_t) = player_q.single() {
        player_t
            .translation()
            .distance(target_transform.translation())
    } else {
        0.0
    };

    **text = format!(
        "Target: {:.0}/{:.0} HP | Dist: {:.0}",
        health.0, max_health.0, distance
    );
}

pub fn update_game_speed_text(
    speed: Res<GameSpeed>,
    paused: Res<GamePaused>,
    auto_target: Res<AutoTarget>,
    hold_fire: Res<HoldFire>,
    mut text_q: Query<&mut Text, With<GameSpeedText>>,
) {
    let Ok(mut text) = text_q.single_mut() else {
        return;
    };

    let speed_str = if paused.0 {
        "PAUSED".to_string()
    } else {
        format!("Speed: x{:.2}", speed.0)
    };

    let auto_str = if auto_target.enabled {
        "[T] Auto-Target: ON"
    } else {
        "[T] Auto-Target: OFF"
    };

    let fire_str = if hold_fire.0 {
        " | [F] HOLD FIRE"
    } else {
        ""
    };

    **text = format!("{} | {}{}", speed_str, auto_str, fire_str);
}
