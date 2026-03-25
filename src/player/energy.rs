//! Energy system: applies power allocation effects to player stats.

use bevy::prelude::*;

use crate::combat::evasion::Evasion;
use crate::combat::shields::Shield;
use crate::combat::tools::TargettedTool;
use crate::game::GameTimeDelta;
use crate::movement::{MaxTurnSpeed, Thrust};

use super::constants::energy::*;
use super::{HoldFire, Player, PlayerBaseStats, SystemPower, WeaponSlot};

/// Applies energy allocation effects each fixed tick:
/// - Shields: regen + max HP scaling
/// - Engines: thrust + turn speed + evasion scaling
/// - Weapons: toggle `armed` on turrets based on power budget
pub fn apply_energy_to_stats(
    power: Res<SystemPower>,
    hold_fire: Res<HoldFire>,
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
    mut weapon_query: Query<(&mut TargettedTool, &WeaponSlot)>,
) {
    let Ok((base_stats, mut shield, mut thrust, mut max_turn, mut evasion)) =
        player_query.get_single_mut()
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

    // Collect weapon slots, sort by index, allocate power
    let mut slots: Vec<(Entity, u32, u32)> = weapon_query
        .iter()
        .map(|(_, ws)| (Entity::PLACEHOLDER, ws.index, ws.power_cost))
        .collect();
    slots.sort_by_key(|s| s.1);

    let mut remaining = weapon_budget;
    let mut powered_indices: Vec<(u32, bool)> = Vec::new();
    for (_, index, cost) in &slots {
        if remaining >= *cost {
            remaining -= cost;
            powered_indices.push((*index, true));
        } else {
            powered_indices.push((*index, false));
        }
    }

    // Apply powered state + hold fire
    for (mut tool, ws) in weapon_query.iter_mut() {
        let should_be_powered = powered_indices
            .iter()
            .find(|(idx, _)| *idx == ws.index)
            .map(|(_, p)| *p)
            .unwrap_or(false);

        tool.armed = should_be_powered && !hold_fire.0;
    }
}
