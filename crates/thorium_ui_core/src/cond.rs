use bevy::{
    ecs::{
        bundle::{BundleEffect, DynamicBundle},
        spawn::SpawnableList,
        system::SystemId,
        world::DeferredWorld,
    },
    prelude::*,
};

use crate::{
    dyn_children::Fragment,
    effect_cell::{AnyEffect, EffectCell},
    owner::Owned,
    Computations, DynChildOf, DynChildren,
};

pub struct Cond<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
    PosChildren: SpawnableList<DynChildOf> + 'static,
    NegChildren: SpawnableList<DynChildOf> + 'static,
    Pos: Fn() -> PosChildren + Send + Sync + 'static,
    Neg: Fn() -> NegChildren + Send + Sync + 'static,
> {
    test_fn: TestFn,
    pos: Pos,
    neg: Neg,
    marker: std::marker::PhantomData<M>,
}

impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosChildren: SpawnableList<DynChildOf> + 'static,
        NegChildren: SpawnableList<DynChildOf> + 'static,
        Pos: Fn() -> PosChildren + Send + Sync + 'static,
        Neg: Fn() -> NegChildren + Send + Sync + 'static,
    > Cond<M, TestFn, PosChildren, NegChildren, Pos, Neg>
{
    pub fn new(test_fn: TestFn, pos: Pos, neg: Neg) -> Self {
        Self {
            test_fn,
            pos,
            neg,
            marker: std::marker::PhantomData,
        }
    }
}

unsafe impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosChildren: SpawnableList<DynChildOf> + Send + Sync + 'static,
        NegChildren: SpawnableList<DynChildOf> + Send + Sync + 'static,
        Pos: Fn() -> PosChildren + Send + Sync + 'static,
        Neg: Fn() -> NegChildren + Send + Sync + 'static,
    > Bundle for Cond<M, TestFn, PosChildren, NegChildren, Pos, Neg>
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

impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosChildren: SpawnableList<DynChildOf> + Send + Sync + 'static,
        NegChildren: SpawnableList<DynChildOf> + Send + Sync + 'static,
        Pos: Fn() -> PosChildren + Send + Sync + 'static,
        Neg: Fn() -> NegChildren + Send + Sync + 'static,
    > DynamicBundle for Cond<M, TestFn, PosChildren, NegChildren, Pos, Neg>
{
    type Effect = Self;

    fn get_components(
        self,
        _func: &mut impl FnMut(bevy::ecs::component::StorageType, bevy::ptr::OwningPtr<'_>),
    ) -> Self::Effect {
        self
    }
}

impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosChildren: SpawnableList<DynChildOf> + Send + Sync + 'static,
        NegChildren: SpawnableList<DynChildOf> + Send + Sync + 'static,
        Pos: Fn() -> PosChildren + Send + Sync + 'static,
        Neg: Fn() -> NegChildren + Send + Sync + 'static,
    > BundleEffect for Cond<M, TestFn, PosChildren, NegChildren, Pos, Neg>
{
    fn apply(self, entity: &mut EntityWorldMut) {
        let test_sys = entity.world_scope(|world| world.register_system(self.test_fn));
        entity.insert((
            EffectCell::new(CondEffect2 {
                state: false,
                first: true,
                test_sys,
                pos: self.pos,
                neg: self.neg,
                marker: std::marker::PhantomData::<M>,
            }),
            // GhostNode::default(),
            Fragment,
        ));
    }
}

/// Conditional control-flow node.
struct CondEffect2<
    M,
    PosChildren: SpawnableList<DynChildOf>,
    Pos: Fn() -> PosChildren,
    NegChildren: SpawnableList<DynChildOf>,
    Neg: Fn() -> NegChildren,
> {
    state: bool,
    first: bool,
    test_sys: SystemId<(), bool>,
    pos: Pos,
    neg: Neg,
    marker: std::marker::PhantomData<M>,
}

impl<
        M,
        PosChildren: SpawnableList<DynChildOf>,
        Pos: Fn() -> PosChildren,
        NegChildren: SpawnableList<DynChildOf>,
        Neg: Fn() -> NegChildren,
    > AnyEffect for CondEffect2<M, PosChildren, Pos, NegChildren, Neg>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        // Run the condition and see if the result changed.
        let test = world.run_system(self.test_sys);
        if let Ok(test) = test {
            if self.state != test || self.first {
                self.first = false;
                let mut entt = world.entity_mut(entity);
                entt.despawn_related::<DynChildren>();
                entt.despawn_related::<Computations>();
                entt.despawn_related::<Owned>();
                if test {
                    (self.pos)().spawn(world, entity);
                } else {
                    (self.neg)().spawn(world, entity);
                }
                self.state = test;
            }
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.test_sys);
    }
}
