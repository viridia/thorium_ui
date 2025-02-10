use core::slice;

use bevy::{
    ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands},
    prelude::*,
};

/// Represents a component that is member of a dynamic child list.
// #[derive(Component, Clone, Reflect, PartialEq, Eq, Debug)]
// #[reflect(Component, PartialEq, Debug, FromWorld)]
#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship(relationship_target = DynChildren)]
pub struct DynChildOf(pub Entity);

impl DynChildOf {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl Default for DynChildOf {
    fn default() -> Self {
        DynChildOf(Entity::PLACEHOLDER)
    }
}

/// "Dynamic Children" are a collection of entities that are used to calculate the children of an
/// entity at runtime. Unlike `Children`, `DynChildren` is not a flat vector, but a hierarchy
/// consisting of normal entities and "fragments", which represent a span of multiple entities.
/// Fragments are entities that have the `Fragment` marker component, and which will have dynamic
/// chilren of their own. Any non-fragment entity which has dynamic children will have it's normal
/// children computed by flattening the hierarchy of dynamic children.
// #[derive(Component, Default, Reflect)]
// #[reflect(Component)]
#[derive(Component, Default)]
#[relationship_target(relationship = DynChildOf, linked_spawn)]
pub struct DynChildren(Vec<Entity>);

impl<'a> IntoIterator for &'a DynChildren {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl core::ops::Deref for DynChildren {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! dyn_children {
    [$($child:expr),*$(,)?] => {
       thorium_ui_core::DynChildren::spawn(($(Spawn($child)),*))
    };
}

/// A type alias over [`RelatedSpawner`] used to spawn child entities containing a [`DynChildOf`]
/// relationship.
pub type DynChildSpawner<'w> = RelatedSpawner<'w, DynChildOf>;

/// A type alias over [`RelatedSpawnerCommands`] used to spawn child entities containing a
/// [`DynChildOf`] relationship.
pub type DynChildSpawnerCommands<'w> = RelatedSpawnerCommands<'w, DynChildOf>;

/// The `Fragment` marker component indicates that this entities contains children which
/// should be spliced into the parent's children list. In other words, when computing the actual
/// children, the fragment entity is replaced by its children.
#[derive(Component, Default)]
pub struct Fragment;

/// If a Fragment entity has changed children, then also mark the non-fragment ancestor as
/// changed. This will ensure that the ancestor's children are recomputed.
pub fn mark_children_changed(
    q_fragments: Query<(Ref<DynChildren>, &DynChildOf), With<Fragment>>,
    mut q_non_fragments: Query<Mut<DynChildren>, Without<Fragment>>,
) {
    for (dyn_children, dyn_parent) in q_fragments.iter() {
        // If the children of the fragment have changed, then...
        if dyn_children.is_changed() {
            let mut parent = dyn_parent.0;
            loop {
                if let Ok(mut parent_dyn_children) = q_non_fragments.get_mut(parent) {
                    // If it's a non-fragment, mark as changed and then stop.
                    parent_dyn_children.set_changed();
                    break;
                } else if let Ok((_, fragment_parent)) = q_fragments.get(parent) {
                    // If it's a fragment, continue up the hierarchy.
                    parent = fragment_parent.0;
                } else {
                    // We've reached the top of the hierarchy, stop.
                    break;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn flatten_dyn_children(
    q_parents: Query<
        (Entity, &DynChildren, Option<Mut<Children>>),
        (Without<Fragment>, Changed<DynChildren>),
    >,
    q_fragments: Query<(Entity, &DynChildren), With<Fragment>>,
    mut commands: Commands,
) {
    for (entity, dyn_children, children) in q_parents.iter() {
        // Use the previous children count to pre-allocate the new children list.
        // This is only a heuristic, to get the real count we'd need to walk the hierarchy.
        let old_count = match children {
            Some(children) => children.len(),
            None => 0,
        };

        // Recursively flatten the hierarchy of dynamic children. If the child is a fragment, we
        // replace it with its children, otherwise push it to the new children list.
        let mut new_children = Vec::<Entity>::with_capacity(old_count);
        flatten(&mut new_children, dyn_children, &q_fragments);

        // Replace the children list of the entity with the new children list.
        // Note that this is not particularly efficient, as children that are carried over from
        // the previous list will be removed and re-added.
        // TODO: Come up with a more efficient alfgorithm.
        let mut entt = commands.entity(entity);
        entt.remove::<Children>().add_children(&new_children);
    }
}

/// Recursively flatten the hierarchy of dynamic children.
fn flatten(
    new_children: &mut Vec<Entity>,
    dyn_children: &DynChildren,
    q_fragments: &Query<(Entity, &DynChildren), With<Fragment>>,
) {
    for child in dyn_children {
        if let Ok((_, dyn_children)) = q_fragments.get(*child) {
            // If the child is a fragment, replace with it's children
            flatten(new_children, dyn_children, q_fragments);
        } else {
            // Otherwise, push the child to the new children list.
            new_children.push(*child);
        }
    }
}
