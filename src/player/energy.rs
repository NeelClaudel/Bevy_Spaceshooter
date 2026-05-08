//! Energy system: applies power allocation effects to player stats.

use bevy::prelude::*;

use crate::combat::evasion::Evasion;
use crate::combat::shields::Shield;
use crate::combat::tools::TargettedTool;
use crate::game::GameTimeDelta;
use crate::movement::{MaxTurnSpeed, Thrust};

use super::constants::energy::*;
use super::{Player, PlayerBaseStats, SystemPower, WeaponEnabled, WeaponSlot};

/// Applies energy allocation effects each fixed tick:
/// - Shields: regen + max HP scaling
/// - Engines: thrust + turn speed + evasion scaling
/// - Weapons: toggle `armed` on turrets based on power budget
pub fn apply_energy_to_stats(
    power: Res<SystemPower>,
    dt: Res<GameTimeDelta>,
    mut player_query: Query<
        (
            &PlayerBaseStats,
            &mut Shield,
            &mut Thrust,
            &mut MaxTurnSpeed,
            &mut Evasion,
        ),
        With<Player>,
    >,
    mut weapon_query: Query<(&mut TargettedTool, &mut WeaponSlot, &WeaponEnabled)>,
) {
    let Ok((base_stats, mut shield, mut thrust, mut max_turn, mut evasion)) =
        player_query.single_mut()
    else {
        return;
    };

    // --- Shields ---
    let shield_cells = power.shields.current as f32;
    let max_shield_hp = shield_cells * SHIELD_HP_PER_CELL;
    let shield_regen = shield_cells * SHIELD_REGEN_PER_CELL;
    shield.health = (shield.health + shield_regen * dt.0).min(max_shield_hp);

    // --- Engines ---
    let engine_cells = power.engines.current as f32;
    let speed_mult = ENGINE_BASE_SPEED_FRACTION + engine_cells * ENGINE_SPEED_BONUS_PER_CELL;
    thrust.0 = base_stats.thrust * speed_mult;
    max_turn.radians_per_second = base_stats.max_turn_speed * speed_mult;
    evasion.base = engine_cells * ENGINE_EVASION_PER_CELL;

    // --- Weapons: power budget determines which are armed ---
    let weapon_budget = power.weapons.current;

    // Allocate power in slot-index order, but only to *enabled* turrets.
    // Disabled turrets are skipped — their cells go to lower-priority turrets.
    // This makes the WeaponEnabled toggle a real power-management tool: turn off
    // a weapon you don't need to free its cells for one further down the list.
    let mut slots: Vec<(u32, u32, bool)> = weapon_query
        .iter()
        .map(|(_, ws, enabled)| (ws.index, ws.power_cost, enabled.0))
        .collect();
    slots.sort_by_key(|s| s.0);

    let mut remaining = weapon_budget;
    let mut powered_indices: Vec<(u32, bool)> = Vec::new();
    for (index, cost, enabled) in &slots {
        if *enabled && remaining >= *cost {
            remaining -= cost;
            powered_indices.push((*index, true));
        } else {
            powered_indices.push((*index, false));
        }
    }

    // Apply powered + enabled state. `armed` requires both; the weapon panel
    // toggles `enabled`, the energy budget controls `powered`.
    for (mut tool, mut ws, enabled) in weapon_query.iter_mut() {
        let powered = powered_indices
            .iter()
            .find(|(idx, _)| *idx == ws.index)
            .map(|(_, p)| *p)
            .unwrap_or(false);
        ws.powered = powered;
        tool.armed = powered && enabled.0;
    }
}
