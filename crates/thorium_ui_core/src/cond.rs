use bevy::{
    ecs::{
        bundle::{BundleEffect, DynamicBundle},
        spawn::SpawnableList,
        system::SystemId,
        world::DeferredWorld,
    },
    prelude::*,
    ui::experimental::GhostNode,
};

use crate::{
    effect_cell::{AnyEffect, EffectCell},
    owner::Owned,
};

pub struct Cond<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
    PosBundle: SpawnableList<ChildOf> + 'static,
    NegBundle: SpawnableList<ChildOf> + 'static,
    Pos: Fn() -> PosBundle + Send + Sync + 'static,
    Neg: Fn() -> NegBundle + Send + Sync + 'static,
> {
    test_fn: TestFn,
    pos: Pos,
    neg: Neg,
    marker: std::marker::PhantomData<M>,
}

impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosBundle: SpawnableList<ChildOf> + 'static,
        NegBundle: SpawnableList<ChildOf> + 'static,
        Pos: Fn() -> PosBundle + Send + Sync + 'static,
        Neg: Fn() -> NegBundle + Send + Sync + 'static,
    > Cond<M, TestFn, PosBundle, NegBundle, Pos, Neg>
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
        PosBundle: SpawnableList<ChildOf> + Send + Sync + 'static,
        NegBundle: SpawnableList<ChildOf> + Send + Sync + 'static,
        Pos: Fn() -> PosBundle + Send + Sync + 'static,
        Neg: Fn() -> NegBundle + Send + Sync + 'static,
    > Bundle for Cond<M, TestFn, PosBundle, NegBundle, Pos, Neg>
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
        PosBundle: SpawnableList<ChildOf> + Send + Sync + 'static,
        NegBundle: SpawnableList<ChildOf> + Send + Sync + 'static,
        Pos: Fn() -> PosBundle + Send + Sync + 'static,
        Neg: Fn() -> NegBundle + Send + Sync + 'static,
    > DynamicBundle for Cond<M, TestFn, PosBundle, NegBundle, Pos, Neg>
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
        PosBundle: SpawnableList<ChildOf> + Send + Sync + 'static,
        NegBundle: SpawnableList<ChildOf> + Send + Sync + 'static,
        Pos: Fn() -> PosBundle + Send + Sync + 'static,
        Neg: Fn() -> NegBundle + Send + Sync + 'static,
    > BundleEffect for Cond<M, TestFn, PosBundle, NegBundle, Pos, Neg>
{
    fn apply(self, entity: &mut EntityWorldMut) {
        let test_sys = unsafe { entity.world_mut().register_system(self.test_fn) };
        entity.insert((
            EffectCell::new(CondEffect2 {
                state: false,
                first: true,
                test_sys,
                pos: self.pos,
                neg: self.neg,
                marker: std::marker::PhantomData::<M>,
            }),
            GhostNode::default(),
        ));
    }
}

pub trait CreateCond2 {
    fn cond2<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosBundle: SpawnableList<ChildOf> + 'static,
        NegBundle: SpawnableList<ChildOf> + 'static,
        Pos: Fn() -> PosBundle + Send + Sync + 'static,
        Neg: Fn() -> NegBundle + Send + Sync + 'static,
    >(
        &mut self,
        test: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self;
}

pub trait CreateCond {
    fn cond<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        Pos: Fn(&mut ChildSpawnerCommands) + Send + Sync + 'static,
        Neg: Fn(&mut ChildSpawnerCommands) + Send + Sync + 'static,
    >(
        &mut self,
        test: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self;
}

impl CreateCond for ChildSpawnerCommands<'_> {
    fn cond<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        Pos: Fn(&mut ChildSpawnerCommands) + Send + Sync + 'static,
        Neg: Fn(&mut ChildSpawnerCommands) + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self {
        // let test_sys = self.commands().register_system(test);
        let mut ent = self.spawn_empty();
        let test_sys = ent.commands().register_system(test_fn);
        ent.insert((
            EffectCell::new(CondEffect {
                state: false,
                first: true,
                test_sys,
                pos,
                neg,
                marker: std::marker::PhantomData::<M>,
            }),
            GhostNode::default(),
        ));
        self
    }
}

impl CreateCond2 for ChildSpawnerCommands<'_> {
    fn cond2<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        PosBundle: SpawnableList<ChildOf> + 'static,
        NegBundle: SpawnableList<ChildOf> + 'static,
        Pos: Fn() -> PosBundle + Send + Sync + 'static,
        Neg: Fn() -> NegBundle + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self {
        // let test_sys = self.commands().register_system(test);
        let mut ent = self.spawn_empty();
        let test_sys = ent.commands().register_system(test_fn);
        ent.insert((
            EffectCell::new(CondEffect2 {
                state: false,
                first: true,
                test_sys,
                pos,
                neg,
                marker: std::marker::PhantomData::<M>,
            }),
            GhostNode::default(),
        ));
        self
    }
}

// impl CreateCond for WorldChildBuilder<'_> {
//     fn cond<
//         M: Send + Sync + 'static,
//         TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
//         Pos: Fn(&mut ChildBuilder) + Send + Sync + 'static,
//         Neg: Fn(&mut ChildBuilder) + Send + Sync + 'static,
//     >(
//         &mut self,
//         test_fn: TestFn,
//         pos: Pos,
//         neg: Neg,
//     ) -> &mut Self {
//         let mut ent = self.spawn_empty();
//         // SAFETFY: Should be safe to register a system here...I think?
//         let test_sys = unsafe { ent.world_mut().register_system(test_fn) };
//         ent.insert((
//             EffectCell::new(CondEffect {
//                 state: false,
//                 first: true,
//                 test_sys,
//                 pos,
//                 neg,
//                 marker: std::marker::PhantomData::<M>,
//             }),
//             GhostNode::default(),
//         ));
//         self
//     }
// }

/// Conditional control-flow node.
struct CondEffect<M, Pos: Fn(&mut ChildSpawnerCommands), Neg: Fn(&mut ChildSpawnerCommands)> {
    state: bool,
    first: bool,
    test_sys: SystemId<(), bool>,
    pos: Pos,
    neg: Neg,
    marker: std::marker::PhantomData<M>,
}

impl<M, Pos: Fn(&mut ChildSpawnerCommands), Neg: Fn(&mut ChildSpawnerCommands)> AnyEffect
    for CondEffect<M, Pos, Neg>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        // Run the condition and see if the result changed.
        let test = world.run_system(self.test_sys);
        if let Ok(test) = test {
            if self.state != test || self.first {
                self.first = false;
                let mut entt = world.entity_mut(entity);
                entt.despawn_related::<Children>();
                entt.despawn_related::<Owned>();
                if test {
                    world.commands().entity(entity).with_children(|builder| {
                        (self.pos)(builder);
                    });
                } else {
                    world.commands().entity(entity).with_children(|builder| {
                        (self.neg)(builder);
                    });
                }
                self.state = test;
            }
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.test_sys);
    }
}

/// Conditional control-flow node.
struct CondEffect2<
    M,
    PosBundle: SpawnableList<ChildOf>,
    Pos: Fn() -> PosBundle,
    NegBundle: SpawnableList<ChildOf>,
    Neg: Fn() -> NegBundle,
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
        PosBundle: SpawnableList<ChildOf>,
        Pos: Fn() -> PosBundle,
        NegBundle: SpawnableList<ChildOf>,
        Neg: Fn() -> NegBundle,
    > AnyEffect for CondEffect2<M, PosBundle, Pos, NegBundle, Neg>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        // Run the condition and see if the result changed.
        let test = world.run_system(self.test_sys);
        if let Ok(test) = test {
            if self.state != test || self.first {
                self.first = false;
                let mut entt = world.entity_mut(entity);
                entt.despawn_related::<Children>();
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
