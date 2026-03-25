use bevy::prelude::*;

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
