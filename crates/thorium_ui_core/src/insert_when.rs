use bevy::{
    ecs::{
        bundle::{BundleEffect, DynamicBundle},
        system::SystemId,
        world::DeferredWorld,
    },
    prelude::*,
};

use crate::{
    effect_cell::{AnyEffect, EffectCell},
    owner::OwnedBy,
};

pub struct InsertWhenEffect<B: Bundle, FactoryFn: Fn() -> B> {
    target: Entity,
    state: bool,
    test_sys: SystemId<(), bool>,
    factory: FactoryFn,
}

impl<B: Bundle, FactoryFn: Fn() -> B> AnyEffect for InsertWhenEffect<B, FactoryFn> {
    fn update(&mut self, world: &mut World, _entity: Entity) {
        // Run the condition and see if the result changed.
        let test = world.run_system(self.test_sys);
        if let Ok(test) = test {
            if self.state != test {
                if test {
                    world
                        .commands()
                        .entity(self.target)
                        .insert((self.factory)());
                } else {
                    world.commands().entity(self.target).remove::<B>();
                }
                self.state = test;
            }
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.test_sys);
    }
}

pub struct InsertWhen<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
    B: Bundle,
    FactoryFn: Fn() -> B + Send + Sync + 'static,
> {
    test_fn: TestFn,
    factory: FactoryFn,
    marker: std::marker::PhantomData<M>,
}

impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    > InsertWhen<M, TestFn, B, FactoryFn>
{
    pub fn new(test_fn: TestFn, factory: FactoryFn) -> Self {
        Self {
            test_fn,
            factory,
            marker: std::marker::PhantomData,
        }
    }
}

unsafe impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    > Bundle for InsertWhen<M, TestFn, B, FactoryFn>
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
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    > DynamicBundle for InsertWhen<M, TestFn, B, FactoryFn>
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
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    > BundleEffect for InsertWhen<M, TestFn, B, FactoryFn>
{
    fn apply(self, parent: &mut EntityWorldMut) {
        let target = parent.id();
        let world = unsafe { parent.world_mut() };
        let test_sys = world.register_system(self.test_fn);
        world.spawn((
            EffectCell::new(InsertWhenEffect {
                target,
                state: false,
                test_sys,
                factory: self.factory,
            }),
            OwnedBy(target),
        ));
    }
}
