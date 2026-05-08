//! Muzzle flares: a one-shot animated effect spawned at the source of any
//! attack/projectile that wants one. Attaches as a marker on the effect entity,
//! and a system fires once when the entity is created.

use bevy::prelude::*;

use crate::combat::effects::SourceTransform;
use crate::combat::CombatSystems;

use super::animated::{AnimatedEffects, CreateAnimatedEffect};

/// Marker on an effect/projectile spawn entity: a muzzle flare of this kind
/// will be spawned at the entity's SourceTransform on the frame it appears.
#[derive(Component)]
pub struct MuzzleFlareEmitter {
    pub effect: AnimatedEffects,
}

pub struct MuzzleFlarePlugin;

impl Plugin for MuzzleFlarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spawn_muzzle_flares.after(CombatSystems));
    }
}

fn spawn_muzzle_flares(
    mut commands: Commands,
    query: Query<(&MuzzleFlareEmitter, &SourceTransform), Added<SourceTransform>>,
) {
    for (emitter, source) in query.iter() {
        let mut transform = source.0.compute_transform();
        transform.translation.z += 0.05;
        commands.spawn(CreateAnimatedEffect {
            effect: emitter.effect,
            transform,
            parent: None,
        });
    }
}
