//! Weapons that can deal damage.
//!

pub mod gatling;

use bevy::prelude::*;

use crate::{
    combat::{attack::Attack, damage::Damage},
    fx::{animated::AnimatedEffects, beams::BeamStyle, muzzle::MuzzleFlareEmitter, HitEffect},
};

/// The attack from a small pulsed laser.
pub fn pulse_laser_attack(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Attack::new(3.0),
            Damage::new(20.0),
            BeamStyle {
                effect: AnimatedEffects::BlueLaserBeam,
                width: 1.0,
            },
            HitEffect {
                effect: AnimatedEffects::SmallExplosion,
            },
            MuzzleFlareEmitter { effect: AnimatedEffects::MuzzleFlare },
        ))
        .id()
}

pub fn small_pulse_laser_attack(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Attack::new(2.0),
            Damage::new(2.0),
            BeamStyle {
                effect: AnimatedEffects::GreenLaserBeam,
                width: 0.5,
            },
            HitEffect {
                effect: AnimatedEffects::TinyPlusExplosion,
            },
            MuzzleFlareEmitter { effect: AnimatedEffects::MuzzleFlare },
        ))
        .id()
}

pub fn small_rocket_attack(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Attack::new(10.0),
            Damage::new(15.0),
            HitEffect {
                effect: AnimatedEffects::FlashExplosion,
            },
        ))
        .id()
}