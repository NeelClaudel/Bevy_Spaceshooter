//! Weapon panel UI: top-right list of player turrets with toggle, name,
//! cooldown bar, and ammo. Click the toggle to enable/disable a weapon.

use bevy::prelude::*;

use crate::combat::tools::{Cooldown, TargettedTool};

use super::constants::hud::*;
use super::{Ammunition, PlayerWeapon, WeaponEnabled, WeaponName, WeaponSlot};

// ---------------------------------------------------------------------------
// Markers
// ---------------------------------------------------------------------------

#[derive(Component)]
struct WeaponPanelRoot;

#[derive(Component)]
struct WeaponPanelRow {
    turret: Entity,
}

#[derive(Component)]
struct WeaponToggleButton {
    turret: Entity,
}

#[derive(Component)]
struct WeaponToggleLabel {
    turret: Entity,
}

#[derive(Component)]
struct WeaponCooldownFill {
    turret: Entity,
}

#[derive(Component)]
struct WeaponAmmoText {
    turret: Entity,
}

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct WeaponPanelPlugin;

impl Plugin for WeaponPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_panel_root);
        app.add_systems(
            Update,
            (
                spawn_rows_for_new_turrets,
                handle_toggle_clicks,
                update_panel_state,
                cleanup_dead_rows,
            ),
        );
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn color_from_arr(c: [f32; 4]) -> Color {
    Color::srgba(c[0], c[1], c[2], c[3])
}

// ---------------------------------------------------------------------------
// Setup
// ---------------------------------------------------------------------------

fn spawn_panel_root(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(HUD_MARGIN),
                top: Val::Px(WEAPON_PANEL_TOP),
                width: Val::Px(WEAPON_PANEL_WIDTH),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(WEAPON_ROW_GAP),
                ..default()
            },
            BackgroundColor(color_from_arr(HUD_PANEL_BG_COLOR)),
            WeaponPanelRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("WEAPONS"),
                TextFont {
                    font_size: HUD_FONT_SIZE,
                    ..default()
                },
                TextColor(color_from_arr(HUD_LABEL_COLOR)),
            ));
        });
}

// ---------------------------------------------------------------------------
// Row spawning (runs whenever a new player turret appears)
// ---------------------------------------------------------------------------

fn spawn_rows_for_new_turrets(
    mut commands: Commands,
    new_turrets: Query<(Entity, &WeaponName, Option<&Ammunition>), Added<PlayerWeapon>>,
    root_q: Query<Entity, With<WeaponPanelRoot>>,
) {
    let Ok(root) = root_q.single() else {
        return;
    };

    for (turret, name, ammo) in new_turrets.iter() {
        let row = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                WeaponPanelRow { turret },
            ))
            .with_children(|row| {
                // Toggle button (clickable). Color set by update_panel_state.
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(WEAPON_TOGGLE_WIDTH),
                        height: Val::Px(WEAPON_TOGGLE_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(color_from_arr(WEAPON_TOGGLE_ON_COLOR)),
                    WeaponToggleButton { turret },
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("ON"),
                        TextFont {
                            font_size: HUD_FONT_SIZE,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        WeaponToggleLabel { turret },
                    ));
                });

                // Weapon name
                row.spawn((
                    Node {
                        width: Val::Px(WEAPON_NAME_WIDTH),
                        ..default()
                    },
                    Text::new(name.0),
                    TextFont {
                        font_size: HUD_FONT_SIZE,
                        ..default()
                    },
                    TextColor(color_from_arr(HUD_LABEL_COLOR)),
                ));

                // Cooldown bar background + fill
                row.spawn((
                    Node {
                        width: Val::Px(WEAPON_COOLDOWN_BAR_WIDTH),
                        height: Val::Px(WEAPON_COOLDOWN_BAR_HEIGHT),
                        ..default()
                    },
                    BackgroundColor(color_from_arr(WEAPON_COOLDOWN_BG_COLOR)),
                ))
                .with_children(|bg| {
                    bg.spawn((
                        Node {
                            width: Val::Px(WEAPON_COOLDOWN_BAR_WIDTH),
                            height: Val::Px(WEAPON_COOLDOWN_BAR_HEIGHT),
                            ..default()
                        },
                        BackgroundColor(color_from_arr(WEAPON_COOLDOWN_FILL_COLOR)),
                        WeaponCooldownFill { turret },
                    ));
                });

                // Ammo text (always present so layout is stable; "" when no ammo).
                let ammo_text = match ammo {
                    Some(a) => format!("{}/{}", a.current, a.max),
                    None => String::new(),
                };
                row.spawn((
                    Node {
                        width: Val::Px(WEAPON_AMMO_WIDTH),
                        ..default()
                    },
                    Text::new(ammo_text),
                    TextFont {
                        font_size: HUD_FONT_SIZE,
                        ..default()
                    },
                    TextColor(color_from_arr(HUD_LABEL_COLOR)),
                    WeaponAmmoText { turret },
                ));
            })
            .id();

        commands.entity(root).add_child(row);
    }
}

// ---------------------------------------------------------------------------
// Click handling
// ---------------------------------------------------------------------------

fn handle_toggle_clicks(
    buttons: Query<(&Interaction, &WeaponToggleButton), Changed<Interaction>>,
    mut enabled_q: Query<&mut WeaponEnabled>,
) {
    for (interaction, btn) in buttons.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut enabled) = enabled_q.get_mut(btn.turret) {
            enabled.0 = !enabled.0;
        }
    }
}

// ---------------------------------------------------------------------------
// State updates (run every frame)
// ---------------------------------------------------------------------------

fn update_panel_state(
    turret_q: Query<(
        &WeaponEnabled,
        &WeaponSlot,
        &Cooldown,
        &TargettedTool,
        Option<&Ammunition>,
    )>,
    mut buttons: Query<(&WeaponToggleButton, &mut BackgroundColor)>,
    mut labels: Query<(&WeaponToggleLabel, &mut Text), Without<WeaponAmmoText>>,
    mut cooldown_fills: Query<(&WeaponCooldownFill, &mut Node)>,
    mut ammo_texts: Query<(&WeaponAmmoText, &mut Text), Without<WeaponToggleLabel>>,
) {
    // Toggle button background color. Order matters: user-disabled wins over
    // unpowered, so a deliberate OFF doesn't get masked by PWR.
    for (btn, mut bg) in buttons.iter_mut() {
        let Ok((enabled, slot, _, _, _)) = turret_q.get(btn.turret) else {
            continue;
        };
        let arr = if !enabled.0 {
            WEAPON_TOGGLE_OFF_COLOR
        } else if !slot.powered {
            WEAPON_TOGGLE_NOPOWER_COLOR
        } else {
            WEAPON_TOGGLE_ON_COLOR
        };
        bg.0 = color_from_arr(arr);
    }

    // Toggle button label
    for (label, mut text) in labels.iter_mut() {
        let Ok((enabled, slot, _, _, _)) = turret_q.get(label.turret) else {
            continue;
        };
        let s = if !enabled.0 {
            "OFF"
        } else if !slot.powered {
            "PWR"
        } else {
            "ON"
        };
        **text = s.to_string();
    }

    // Cooldown fill: width = (1 - remaining/duration) of full bar.
    for (fill, mut node) in cooldown_fills.iter_mut() {
        let Ok((_, _, cooldown, _, _)) = turret_q.get(fill.turret) else {
            continue;
        };
        let progress = if cooldown.duration > 0.0 {
            (1.0 - cooldown.remaining / cooldown.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };
        node.width = Val::Px(WEAPON_COOLDOWN_BAR_WIDTH * progress);
    }

    // Ammo text
    for (ammo_marker, mut text) in ammo_texts.iter_mut() {
        let Ok((_, _, _, _, ammo)) = turret_q.get(ammo_marker.turret) else {
            continue;
        };
        let s = match ammo {
            Some(a) => format!("{}/{}", a.current, a.max),
            None => String::new(),
        };
        **text = s;
    }
}

// ---------------------------------------------------------------------------
// Despawn rows whose turret no longer exists (e.g. ship destroyed).
// ---------------------------------------------------------------------------

fn cleanup_dead_rows(
    mut commands: Commands,
    rows: Query<(Entity, &WeaponPanelRow)>,
    turrets: Query<(), With<PlayerWeapon>>,
) {
    for (row_entity, row) in rows.iter() {
        if turrets.get(row.turret).is_err() {
            commands.entity(row_entity).despawn();
        }
    }
}
