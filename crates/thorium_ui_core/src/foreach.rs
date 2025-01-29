use std::ops::Range;

use bevy::{
    ecs::{
        bundle::{BundleEffect, DynamicBundle},
        system::SystemId,
    },
    prelude::*,
    ui::experimental::GhostNode,
};

use crate::{
    dyn_children::Fragment,
    effect_cell::{AnyEffect, EffectCell},
    lcs::lcs,
    DynChildOf, DynChildren, SpawnableListGen, TemplateContext,
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
    EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
    FallbackFn: SpawnableListGen + Send + Sync + 'static,
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
        Item: Clone + Send + Sync + 'static,
        CmpFn: Fn(&Item, &Item) -> bool + Send + Sync + 'static,
        EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
        FallbackFn: SpawnableListGen + Send + Sync + 'static,
    > ForEachEffect<'_, Item, CmpFn, EachFn, FallbackFn>
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
                world.entity_mut(prev.child).despawn();
            }
            // Build new elements
            for i in next_range {
                let child_id = world.spawn((GhostNode::default(), Fragment)).id();
                let mut tc = TemplateContext::new(child_id, world);
                (self.each)(&next_items[i], &mut tc);
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
                    world.entity_mut(prev.child).despawn();
                }
            }
        } else if next_start > next_range.start {
            // Insertions
            for i in next_range.start..next_start {
                let child_id = world.spawn((GhostNode::default(), Fragment)).id();
                let mut tc = TemplateContext::new(child_id, world);
                (self.each)(&next_items[i], &mut tc);
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
                    world.entity_mut(prev.child).despawn();
                }
            }
        } else if next_end < next_range.end {
            // Insertions
            for i in next_end..next_range.end {
                let child_id = world.spawn((GhostNode::default(), Fragment)).id();
                let mut tc = TemplateContext::new(child_id, world);
                (self.each)(&next_items[i], &mut tc);
                out.push(ListItem {
                    child: child_id,
                    item: next_items[i].clone(),
                });
            }
        }
    }
}

impl<
        Item: Clone + Send + Sync + 'static,
        CmpFn: Fn(&Item, &Item) -> bool + Send + Sync + 'static,
        EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
        FallbackFn: SpawnableListGen + Send + Sync + 'static,
    > AnyEffect for ForEachEffect<'_, Item, CmpFn, EachFn, FallbackFn>
{
    fn update(&mut self, world: &mut World, parent: Entity) {
        // Create a reactive context and call the test condition.
        let mut items = ListItems::<Item> {
            items: Vec::new(),
            changed: false,
        };
        world.run_system_with(self.item_sys, &mut items).unwrap();
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
                    world.entity_mut(parent).despawn_related::<DynChildren>();
                    self.fallback.spawn(world, parent);
                }
            } else {
                if prev_len == 0 {
                    // Transitioning from non-empty to empty, delete fallback.
                    world.entity_mut(parent).despawn_related::<DynChildren>();
                }
                world.entity_mut(parent).remove::<DynChildren>();
                world
                    .entity_mut(parent)
                    .add_related::<DynChildOf>(&children);
            }
        }
    }

    fn cleanup(&self, world: &mut bevy::ecs::world::DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.item_sys);
    }
}

pub struct For<
    'a: 'static,
    M: Send + Sync + 'static,
    Item: Send + Sync + 'static + Clone,
    // CmpFn: Send + Sync + 'static + Fn(&Item, &Item) -> bool,
    ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
    EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
    FallbackFn: SpawnableListGen + Send + Sync + 'static,
> {
    items_fn: ItemFn,
    cmp: fn(&Item, &Item) -> bool,
    each: EachFn,
    fallback: FallbackFn,
    marker: std::marker::PhantomData<(&'a M, Item)>,
}

impl<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone,
        // CmpFn: Send + Sync + 'static + Fn(&Item, &Item) -> bool,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
        FallbackFn: SpawnableListGen + Send + Sync + 'static,
    > For<'a, M, Item, ItemFn, EachFn, FallbackFn>
{
    pub fn each(items_fn: ItemFn, each: EachFn, fallback: FallbackFn) -> Self
    where
        Item: PartialEq,
    {
        Self {
            items_fn,
            cmp: PartialEq::eq,
            each,
            fallback,
            marker: std::marker::PhantomData,
        }
    }

    pub fn each_cmp(
        items_fn: ItemFn,
        cmp: fn(&Item, &Item) -> bool,
        each: EachFn,
        fallback: FallbackFn,
    ) -> Self {
        Self {
            items_fn,
            cmp,
            each,
            fallback,
            marker: std::marker::PhantomData,
        }
    }
}

impl<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
        FallbackFn: SpawnableListGen + Send + Sync + 'static,
    > BundleEffect for For<'a, M, Item, ItemFn, EachFn, FallbackFn>
{
    fn apply(self, entity: &mut EntityWorldMut) {
        let item_sys = entity.world_scope(|world| world.register_system(self.items_fn));
        entity.insert((
            EffectCell::new(ForEachEffect {
                item_sys,
                cmp: self.cmp,
                each: self.each,
                fallback: self.fallback,
                state: Vec::new(),
                first: true,
            }),
            // GhostNode::default(),
            Fragment,
        ));
    }
}

impl<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
        FallbackFn: SpawnableListGen + Send + Sync + 'static,
    > DynamicBundle for For<'a, M, Item, ItemFn, EachFn, FallbackFn>
{
    type Effect = Self;

    fn get_components(
        self,
        _func: &mut impl FnMut(bevy::ecs::component::StorageType, bevy::ptr::OwningPtr<'_>),
    ) -> Self::Effect {
        self
    }
}

unsafe impl<
        'a: 'static,
        M: Send + Sync + 'static,
        Item: Send + Sync + 'static + Clone,
        ItemFn: IntoSystem<InMut<'a, ListItems<Item>>, (), M> + Send + Sync + 'static,
        EachFn: Fn(&Item, &mut TemplateContext) + Send + Sync + 'static,
        FallbackFn: SpawnableListGen + Send + Sync + 'static,
    > Bundle for For<'a, M, Item, ItemFn, EachFn, FallbackFn>
{
    fn component_ids(
        _components: &mut bevy::ecs::component::Components,
        _storages: &mut bevy::ecs::storage::Storages,
        _ids: &mut impl FnMut(bevy::ecs::component::ComponentId),
    ) {
    }

    fn get_component_ids(
        _components: &bevy::ecs::component::Components,
        _ids: &mut impl FnMut(Option<bevy::ecs::component::ComponentId>),
    ) {
    }

    fn register_required_components(
        _components: &mut bevy::ecs::component::Components,
        _storages: &mut bevy::ecs::storage::Storages,
        _required_components: &mut bevy::ecs::component::RequiredComponents,
    ) {
    }
}
