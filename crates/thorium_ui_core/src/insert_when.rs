use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::effect_cell::{AnyEffect, ConstructEffect, EffectCell};

pub struct InsertWhenEffect<B: Bundle, FactoryFn: Fn() -> B> {
    target: Entity,
    state: bool,
    test_sys: SystemId<(), bool>,
    factory: FactoryFn,
}

impl<B: Bundle, FactoryFn: Fn() -> B> AnyEffect for InsertWhenEffect<B, FactoryFn> {
    fn update(&mut self, world: &mut World, entity: Entity) {
        // Run the condition and see if the result changed.
        let test = world.run_system(self.test_sys);
        if let Ok(test) = test {
            if self.state != test {
                let mut entt = world.entity_mut(entity);
                entt.despawn_descendants();
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

impl<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    > ConstructEffect for InsertWhen<M, TestFn, B, FactoryFn>
{
    fn construct(self, parent: &mut EntityCommands<'_>) {
        let test_sys = parent.commands().register_system(self.test_fn);
        let target = parent.id();
        parent
            .commands()
            .spawn(EffectCell::new(InsertWhenEffect {
                target,
                state: false,
                test_sys,
                factory: self.factory,
            }))
            .set_parent(target);
    }
}
