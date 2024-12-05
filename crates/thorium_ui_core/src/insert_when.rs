use std::sync::{Arc, Mutex};

use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::effect_cell::{AnyEffect, EffectCell};

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

pub trait InsertWhen {
    fn insert_when<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        factory: FactoryFn,
    ) -> &mut Self;
}

impl InsertWhen for EntityCommands<'_> {
    fn insert_when<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        B: Bundle,
        FactoryFn: Fn() -> B + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        factory: FactoryFn,
    ) -> &mut Self {
        let test_sys = self.commands().register_system(test_fn);
        let target = self.id();
        self.commands()
            .spawn(EffectCell(Arc::new(Mutex::new(InsertWhenEffect {
                target,
                state: false,
                test_sys,
                factory,
            }))))
            .set_parent(target);
        self
    }
}
