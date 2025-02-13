use bevy::prelude::*;

/// The `Fragment` marker component indicates that this entities contains children which
/// should be spliced into the parent's children list. In other words, when computing the actual
/// children, the fragment entity is replaced by its children.
#[derive(Component, Default)]
pub struct Fragment;

/// If a Fragment entity has changed children, then also mark the non-fragment ancestor as
/// changed. This will ensure that the ancestor's children are recomputed.
pub fn mark_children_changed(
    q_fragments: Query<(Ref<Children>, &ChildOf), With<Fragment>>,
    mut q_non_fragments: Query<Mut<Children>, Without<Fragment>>,
) {
    for (children, parent) in q_fragments.iter() {
        // If the children of the fragment have changed, then...
        if children.is_changed() {
            let mut parent = parent.0;
            loop {
                if let Ok(mut non_fragment_parent) = q_non_fragments.get_mut(parent) {
                    // If it's a non-fragment, mark as changed and then stop.
                    non_fragment_parent.set_changed();
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

// #[allow(clippy::type_complexity)]
// pub fn flatten_dyn_children(
//     q_parents: Query<
//         (Entity, &Children, Option<Mut<Children>>),
//         (Without<Fragment>, Changed<Children>),
//     >,
//     q_fragments: Query<(Entity, &Children), With<Fragment>>,
//     mut commands: Commands,
// ) {
//     for (entity, dyn_children, children) in q_parents.iter() {
//         // Use the previous children count to pre-allocate the new children list.
//         // This is only a heuristic, to get the real count we'd need to walk the hierarchy.
//         let old_count = match children {
//             Some(children) => children.len(),
//             None => 0,
//         };

//         // Recursively flatten the hierarchy of dynamic children. If the child is a fragment, we
//         // replace it with its children, otherwise push it to the new children list.
//         let mut new_children = Vec::<Entity>::with_capacity(old_count);
//         flatten(&mut new_children, dyn_children, &q_fragments);

//         // Replace the children list of the entity with the new children list.
//         // Note that this is not particularly efficient, as children that are carried over from
//         // the previous list will be removed and re-added.
//         // TODO: Come up with a more efficient alfgorithm.
//         let mut entt = commands.entity(entity);
//         entt.remove::<Children>().add_children(&new_children);
//     }
// }

// / Recursively flatten the hierarchy of dynamic children.
// fn flatten(
//     new_children: &mut Vec<Entity>,
//     children: &Children,
//     q_fragments: &Query<(Entity, &Children), With<Fragment>>,
// ) {
//     for child in children {
//         if let Ok((_, fragment_children)) = q_fragments.get(*child) {
//             // If the child is a fragment, replace with it's children
//             flatten(new_children, fragment_children, q_fragments);
//         } else {
//             // Otherwise, push the child to the new children list.
//             new_children.push(*child);
//         }
//     }
// }
