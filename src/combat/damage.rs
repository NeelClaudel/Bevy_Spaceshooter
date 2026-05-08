use bevy::prelude::*;

use super::{
    attack::{Attack, AttackResult},
    effects::Effect,
    mortal::{Dieing, Health},
    Target,
};

/// Entity will deal a specified amount of damage.
#[derive(Component)]
pub struct Damage(pub f32);

impl Damage {
    pub fn new(damage: f32) -> Self {
        Damage(damage)
    }
}

/// Tracks when damage was last dealt to this entity.
#[derive(Component)]
pub struct LastDamageTimer(pub f32);

/// Applies damage effects to entities.
///
/// Dying entities are filtered out so their HP doesn't drift below 0 and the
/// AI can't keep stacking hits on a wreck mid-explosion. Surviving hits clamp
/// HP at 0 — the next `check_for_dieing_entities` tick converts that to a
/// `Dieing` component which then gates further damage entirely.
pub fn apply_damage(
    query: Query<(&Target, &Damage, &Attack), With<Effect>>,
    mut health_query: Query<(&mut Health, &mut LastDamageTimer), Without<Dieing>>,
) {
    for (target, damage, attack) in query.iter() {
        if attack.result != AttackResult::Hit {
            continue;
        }

        if let Some(target_entity) = target.0 {
            if let Ok((mut health, mut timer)) = health_query.get_mut(target_entity) {
                health.0 = (health.0 - damage.0).max(0.0);
                timer.0 = 0.0;
            }
        }
    }
}
