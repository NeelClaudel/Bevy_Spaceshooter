use bevy::prelude::*;

use super::mortal::Dieing;

#[derive(Clone, Copy, Component)]
#[derive(Default)]
pub struct Target(pub Option<Entity>);

/// Indicates that an entity should use the target chosen by it's parent.
#[derive(Clone, Copy, Component, Default)]
pub struct InheritTargetFromParent;

pub fn copy_targets_from_parents(
     query: Query<(Entity, &ChildOf), With<InheritTargetFromParent>>,
    mut targetter_query: Query<&mut Target>
) {
    for (entity, child_of) in query.iter() {
        let parent_target = match targetter_query.get(child_of.parent()) {
            Ok(opt) => opt.0,
            Err(_) => None
        };
        if let Ok(mut my_target) = targetter_query.get_mut(entity) {
            my_target.0 = parent_target;
        }
    }
}

/// Nulls any `Target.0` that references an entity which has entered the
/// `Dieing` state. Runs after `copy_targets_from_parents` so children inherit
/// the cleared value from their parent the next tick. Without this, AI ships
/// and player turrets keep firing at a wreck for the rest of its death-throes
/// window — which is just confusing, even if `apply_damage` already filters
/// out further damage.
pub fn clear_targets_pointing_at_dieing(
    mut targetters: Query<&mut Target>,
    dieing: Query<(), With<Dieing>>,
) {
    for mut target in targetters.iter_mut() {
        if let Some(e) = target.0 {
            if dieing.get(e).is_ok() {
                target.0 = None;
            }
        }
    }
}
