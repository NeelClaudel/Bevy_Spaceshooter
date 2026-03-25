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
    Color::rgba(c[0], c[1], c[2], c[3])
}

fn bar_background(width: f32, height: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
        background_color: BackgroundColor(color_from_arr(BAR_BACKGROUND_COLOR)),
        ..default()
    }
}

fn bar_fill(color: [f32; 4], width: f32, height: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
        background_color: BackgroundColor(color_from_arr(color)),
        ..default()
    }
}

pub fn spawn_hud(mut commands: Commands) {
    // --- Bottom-left panel: health, shield, energy ---
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(HUD_MARGIN),
                bottom: Val::Px(HUD_MARGIN),
                padding: UiRect::all(Val::Px(HUD_PADDING)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },
            background_color: BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
            ..default()
        })
        .with_children(|parent| {
            // Health label + bar
            parent.spawn(TextBundle::from_section(
                "HP",
                TextStyle {
                    font_size: HUD_FONT_SIZE,
                    color: color_from_arr(HUD_LABEL_COLOR),
                    ..default()
                },
            ));
            parent
                .spawn(bar_background(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(HEALTH_BAR_COLOR, HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT))
                        .insert(HealthBarFill);
                });

            // Shield label + bar
            parent.spawn(TextBundle::from_section(
                "SHIELD",
                TextStyle {
                    font_size: HUD_FONT_SIZE,
                    color: color_from_arr(HUD_LABEL_COLOR),
                    ..default()
                },
            ));
            parent
                .spawn(bar_background(SHIELD_BAR_WIDTH, SHIELD_BAR_HEIGHT))
                .with_children(|bg| {
                    bg.spawn(bar_fill(SHIELD_BAR_COLOR, SHIELD_BAR_WIDTH, SHIELD_BAR_HEIGHT))
                        .insert(ShieldBarFill);
                });

            // Energy: Shields
            parent.spawn(TextBundle::from_section(
                "1: Shields",
                TextStyle {
                    font_size: HUD_FONT_SIZE,
                    color: color_from_arr(ENERGY_SHIELD_COLOR),
                    ..default()
                },
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
            parent.spawn(TextBundle::from_section(
                "2: Weapons",
                TextStyle {
                    font_size: HUD_FONT_SIZE,
                    color: color_from_arr(ENERGY_WEAPON_COLOR),
                    ..default()
                },
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
            parent.spawn(TextBundle::from_section(
                "3: Engines",
                TextStyle {
                    font_size: HUD_FONT_SIZE,
                    color: color_from_arr(ENERGY_ENGINE_COLOR),
                    ..default()
                },
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
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(HUD_MARGIN),
                top: Val::Px(HUD_MARGIN),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Speed: x1.0",
                    TextStyle {
                        font_size: HUD_FONT_SIZE,
                        color: color_from_arr(HUD_LABEL_COLOR),
                        ..default()
                    },
                ))
                .insert(GameSpeedText);
        });

    // --- Top-right: Target info ---
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(HUD_MARGIN),
                top: Val::Px(HUD_MARGIN),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            background_color: BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "No target",
                    TextStyle {
                        font_size: HUD_FONT_SIZE,
                        color: color_from_arr(TARGET_INDICATOR_COLOR),
                        ..default()
                    },
                ))
                .insert(TargetInfoText);
        });
}

// ---------------------------------------------------------------------------
// Update systems
// ---------------------------------------------------------------------------

pub fn update_health_bar(
    player_q: Query<(&Health, &MaxHealth), With<Player>>,
    mut bar_q: Query<&mut Style, With<HealthBarFill>>,
) {
    let Ok((health, max_health)) = player_q.get_single() else {
        return;
    };
    let Ok(mut style) = bar_q.get_single_mut() else {
        return;
    };
    let fraction = (health.0 / max_health.0).clamp(0.0, 1.0);
    style.width = Val::Px(HEALTH_BAR_WIDTH * fraction);
}

pub fn update_shield_bar(
    player_q: Query<(&Shield, &MaxShieldHP), With<Player>>,
    mut bar_q: Query<&mut Style, With<ShieldBarFill>>,
) {
    let Ok((shield, max_shield)) = player_q.get_single() else {
        return;
    };
    let Ok(mut style) = bar_q.get_single_mut() else {
        return;
    };
    let fraction = (shield.health / max_shield.0.max(1.0)).clamp(0.0, 1.0);
    style.width = Val::Px(SHIELD_BAR_WIDTH * fraction);
}

pub fn update_energy_bars(
    power: Res<SystemPower>,
    mut shields_q: Query<&mut Style, (With<EnergyBarShields>, Without<EnergyBarWeapons>, Without<EnergyBarEngines>)>,
    mut weapons_q: Query<&mut Style, (With<EnergyBarWeapons>, Without<EnergyBarShields>, Without<EnergyBarEngines>)>,
    mut engines_q: Query<&mut Style, (With<EnergyBarEngines>, Without<EnergyBarShields>, Without<EnergyBarWeapons>)>,
) {
    if let Ok(mut style) = shields_q.get_single_mut() {
        let frac = power.shields.current as f32 / power.shields.max_level.max(1) as f32;
        style.width = Val::Px(ENERGY_BAR_WIDTH * frac);
    }
    if let Ok(mut style) = weapons_q.get_single_mut() {
        let frac = power.weapons.current as f32 / power.weapons.max_level.max(1) as f32;
        style.width = Val::Px(ENERGY_BAR_WIDTH * frac);
    }
    if let Ok(mut style) = engines_q.get_single_mut() {
        let frac = power.engines.current as f32 / power.engines.max_level.max(1) as f32;
        style.width = Val::Px(ENERGY_BAR_WIDTH * frac);
    }
}

pub fn update_target_info(
    player_target: Res<PlayerTarget>,
    player_q: Query<&GlobalTransform, With<Player>>,
    target_q: Query<(&Health, &MaxHealth, &GlobalTransform)>,
    mut text_q: Query<&mut Text, With<TargetInfoText>>,
) {
    let Ok(mut text) = text_q.get_single_mut() else {
        return;
    };

    let Some(target_entity) = player_target.0 else {
        text.sections[0].value = "No target".to_string();
        return;
    };

    let Ok((health, max_health, target_transform)) = target_q.get(target_entity) else {
        text.sections[0].value = "Target lost".to_string();
        return;
    };

    let distance = if let Ok(player_t) = player_q.get_single() {
        player_t
            .translation()
            .distance(target_transform.translation())
    } else {
        0.0
    };

    text.sections[0].value = format!(
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
    let Ok(mut text) = text_q.get_single_mut() else {
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

    text.sections[0].value = format!("{} | {}{}", speed_str, auto_str, fire_str);
}
