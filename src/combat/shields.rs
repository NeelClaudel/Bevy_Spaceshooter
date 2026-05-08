//! Implementation of shields.

use bevy::prelude::*;

use crate::fx::animated::{AnimatedEffects, CreateAnimatedEffect};

use super::{
    attack::{Attack, AttackResult},
    damage::Damage,
    effects::{EffectLocation, SourceTransform},
    Target,
};

#[derive(Component)]
pub struct MaxShieldHP(pub f32);
#[derive(Component)]
pub struct Shield {
    pub health: f32,
    pub radius: f32,
}

/// Flag component that indicates an attack bypasses shields.
pub struct BypassShield;

pub fn shield_absorb_damage(
    mut commands: Commands,
    mut attacks_query: Query<(
        &mut Damage,
        &Target,
        &SourceTransform,
        &mut EffectLocation,
        &mut Attack,
        Option<&super::effects::Instigator>,
    )>,
    mut shields_query: Query<(&mut Shield, &GlobalTransform)>,
    instigator_transforms: Query<&GlobalTransform>,
) {
    for (mut damage, target, source_t, mut hit_loc, mut attack, instigator_opt) in
        attacks_query.iter_mut()
    {
        // does attack target have a shield?
        if target.0.is_none() {
            continue;
        }

        // non hit attacks cannot be shielded
        if attack.result != AttackResult::Hit {
            continue;
        }

        if let Ok((mut shield, shield_transform)) =
            shields_query.get_mut(target.0.expect("target is none"))
        {
            // Use the original instigator's position (the ship that fired) if available,
            // otherwise fall back to the effect source. This prevents projectiles from
            // bypassing shields just because they detonated inside the shield radius.
            let attacker_pos = instigator_opt
                .and_then(|ins| instigator_transforms.get(ins.0).ok())
                .map(|t| t.translation())
                .unwrap_or_else(|| source_t.0.translation());

            let delta = attacker_pos - hit_loc.0;

            // if attacker is within shield radius, no protection given:
            if delta.length_squared() < shield.radius.powi(2) {
                continue;
            }

            // shield blocks incoming damage
            let absorbed = shield.health.min(damage.0);

            if absorbed > 0.0 {
                shield.health -= absorbed;
                damage.0 -= absorbed;
                hit_loc.0 += delta.normalize() * shield.radius;
                attack.result = AttackResult::Blocked;

                // spawn a 'hit shield' effect
                commands.spawn(
                    CreateAnimatedEffect {
                        transform: Transform::from_translation(shield_transform.translation())
                            * Transform::from_rotation(Quat::from_rotation_z(
                                delta.y.atan2(delta.x) - std::f32::consts::FRAC_PI_2,
                            ))
                            * Transform::from_scale(Vec3::splat(shield.radius / 32.0)),
                        parent: None,
                        effect: AnimatedEffects::Shield,
                    },
                );
            }
        }
    }
}
