use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

use bevy::{ecs::system::SystemId, prelude::*, ui::experimental::GhostNode};

use crate::{
    effect_cell::{AnyEffect, EffectCell, UnregisterSystemCommand},
    lcs::lcs,
};

pub struct ListItems<Item: Clone> {
    items: Vec<Item>,
    changed: bool,
}

impl<Item: Clone> ListItems<Item> {
    pub fn clone_from(&mut self, items: &Vec<Item>) {
        self.items.clone_from(items);
        self.changed = true;
    }

    pub fn clone_from_iter<I: Iterator<Item = Item>>(&mut self, items: I) {
        self.items.extend(items);
        self.changed = true;
    }

    pub fn push(&mut self, item: Item) {
        self.items.push(item);
        self.changed = true;
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.changed = true;
    }
}

pub trait CreateForEach {
    fn for_each<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone + PartialEq,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
        FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        items_fn: ItemFn,
        each: EachFn,
        fallback: FallbackFn,
    ) -> &mut Self;

    fn for_each_cmp<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone,
        CmpFn: Send + Sync + 'static + Fn(&Item, &Item) -> bool,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
        FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        items_fn: ItemFn,
        cmp: CmpFn,
        each: EachFn,
        fallback: FallbackFn,
    ) -> &mut Self;
}

impl CreateForEach for ChildBuilder<'_> {
    fn for_each<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone + PartialEq,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
        FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        items_fn: ItemFn,
        each: EachFn,
        fallback: FallbackFn,
    ) -> &mut Self {
        let mut ent = self.spawn_empty();
        let item_sys = ent.commands().register_system(items_fn);
        ent.insert(EffectCell(Arc::new(Mutex::new(ForEachEffect {
            item_sys,
            cmp: PartialEq::eq,
            each,
            fallback,
            state: Vec::new(),
            first: true,
        }))));
        self
    }

    fn for_each_cmp<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone,
        CmpFn: Send + Sync + 'static + Fn(&Item, &Item) -> bool,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
        FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        items_fn: ItemFn,
        cmp: CmpFn,
        each: EachFn,
        fallback: FallbackFn,
    ) -> &mut Self {
        let mut ent = self.spawn_empty();
        let item_sys = ent.commands().register_system(items_fn);
        ent.insert(EffectCell(Arc::new(Mutex::new(ForEachEffect {
            item_sys,
            cmp,
            each,
            fallback,
            state: Vec::new(),
            first: true,
        }))));
        self
    }
}

#[derive(Clone)]
struct ListItem<Item: Clone> {
    child: Entity,
    item: Item,
}

/// A reaction that handles the conditional rendering logic.
struct ForEachEffect<
    'a,
    Item: Clone + 'static,
    CmpFn: Fn(&Item, &Item) -> bool,
    EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
    FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
> where
    Self: Send + Sync,
{
    item_sys: SystemId<InMut<'a, ListItems<Item>>, ()>,
    cmp: CmpFn,
    each: EachFn,
    fallback: FallbackFn,
    state: Vec<ListItem<Item>>,
    first: bool,
}

impl<
        'a,
        Item: Clone + Send + Sync + 'static,
        CmpFn: Fn(&Item, &Item) -> bool + Send + Sync + 'static,
        EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
        FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    > ForEachEffect<'a, Item, CmpFn, EachFn, FallbackFn>
{
    /// Uses the sequence of key values to match the previous array items with the updated
    /// array items. Matching items are patched, other items are inserted or deleted.
    ///
    /// # Arguments
    ///
    /// * `bc` - [`BuildContext`] used to build individual elements.
    /// * `prev_state` - Array of view state elements from previous update.
    /// * `prev_range` - The range of elements we are comparing in `prev_state`.
    /// * `next_state` - Array of view state elements to be built.
    /// * `next_range` - The range of elements we are comparing in `next_state`.
    #[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
    fn build_recursive(
        &self,
        world: &mut World,
        prev_state: &[ListItem<Item>],
        prev_range: Range<usize>,
        next_items: &[Item],
        next_range: Range<usize>,
        out: &mut Vec<ListItem<Item>>,
    ) {
        // Look for longest common subsequence.
        // prev_start and next_start are *relative to the slice*.
        let (prev_start, next_start, lcs_length) = lcs(
            &prev_state[prev_range.clone()],
            &next_items[next_range.clone()],
            |a, b| (self.cmp)(&a.item, b),
        );

        // If there was nothing in common
        if lcs_length == 0 {
            // Raze old elements
            for i in prev_range {
                let prev = &prev_state[i];
                world.entity_mut(prev.child).despawn_recursive();
            }
            // Build new elements
            for i in next_range {
                let child_id = world.spawn(GhostNode::default()).id();
                world.commands().entity(child_id).with_children(|builder| {
                    (self.each)(&next_items[i], builder);
                });
                out.push(ListItem {
                    child: child_id,
                    item: next_items[i].clone(),
                });
            }
            return;
        }

        // Adjust prev_start and next_start to be relative to the entire state array.
        let prev_start = prev_start + prev_range.start;
        let next_start = next_start + next_range.start;

        // Stuff that precedes the LCS.
        if prev_start > prev_range.start {
            if next_start > next_range.start {
                // Both prev and next have entries before lcs, so recurse
                self.build_recursive(
                    world,
                    // owner,
                    prev_state,
                    prev_range.start..prev_start,
                    next_items,
                    next_range.start..next_start,
                    out,
                )
            } else {
                // Deletions
                for i in prev_range.start..prev_start {
                    let prev = &prev_state[i];
                    world.entity_mut(prev.child).despawn_recursive();
                }
            }
        } else if next_start > next_range.start {
            // Insertions
            for i in next_range.start..next_start {
                let child_id = world.spawn(GhostNode::default()).id();
                world.commands().entity(child_id).with_children(|builder| {
                    (self.each)(&next_items[i], builder);
                });
                out.push(ListItem {
                    child: child_id,
                    item: next_items[i].clone(),
                });
            }
        }

        // For items that match, copy over the view and value.
        for i in 0..lcs_length {
            let prev = &prev_state[prev_start + i];
            out.push(prev.clone());
        }

        // Stuff that follows the LCS.
        let prev_end = prev_start + lcs_length;
        let next_end = next_start + lcs_length;
        if prev_end < prev_range.end {
            if next_end < next_range.end {
                // Both prev and next have entries after lcs, so recurse
                self.build_recursive(
                    world,
                    // owner,
                    prev_state,
                    prev_end..prev_range.end,
                    next_items,
                    next_end..next_range.end,
                    out,
                );
            } else {
                // Deletions
                for i in prev_end..prev_range.end {
                    let prev = &prev_state[i];
                    world.entity_mut(prev.child).despawn_recursive();
                }
            }
        } else if next_end < next_range.end {
            // Insertions
            for i in next_end..next_range.end {
                let child_id = world.spawn(GhostNode::default()).id();
                world.commands().entity(child_id).with_children(|builder| {
                    (self.each)(&next_items[i], builder);
                });
                out.push(ListItem {
                    child: child_id,
                    item: next_items[i].clone(),
                });
            }
        }
    }
}

impl<
        'a,
        Item: Clone + Send + Sync + 'static,
        CmpFn: Fn(&Item, &Item) -> bool + Send + Sync + 'static,
        EachFn: Send + Sync + 'static + Fn(&Item, &mut ChildBuilder),
        FallbackFn: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    > AnyEffect for ForEachEffect<'a, Item, CmpFn, EachFn, FallbackFn>
{
    fn update(&mut self, world: &mut World, parent: Entity) {
        // Create a reactive context and call the test condition.
        let mut items = ListItems::<Item> {
            items: Vec::new(),
            changed: false,
        };
        world
            .run_system_with_input(self.item_sys, &mut items)
            .unwrap();
        if items.changed || self.first {
            let mut next_state: Vec<ListItem<Item>> = Vec::with_capacity(items.items.len());
            let next_len = items.items.len();
            let prev_len = self.state.len();

            self.build_recursive(
                world,
                &self.state,
                0..prev_len,
                &items.items,
                0..next_len,
                &mut next_state,
            );
            let children: Vec<Entity> = next_state.iter().map(|i| i.child).collect();
            self.state = std::mem::take(&mut next_state);

            if next_len == 0 {
                if prev_len > 0 || self.first {
                    self.first = false;
                    // Transitioning from non-empty to empty, generate fallback.
                    world.entity_mut(parent).despawn_descendants();
                    world.commands().entity(parent).with_children(|builder| {
                        (self.fallback)(builder);
                    });
                }
            } else {
                if prev_len == 0 {
                    // Transitioning from non-empty to empty, delete fallback.
                    world.entity_mut(parent).despawn_descendants();
                }
                world.entity_mut(parent).replace_children(&children);
            }
        }
    }

    fn cleanup(&self, world: &mut bevy::ecs::world::DeferredWorld, _entity: Entity) {
        world
            .commands()
            .queue(UnregisterSystemCommand(self.item_sys));
    }
}
