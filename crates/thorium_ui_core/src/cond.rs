use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::{
    effect_cell::{AnyEffect, EffectCell, UnregisterSystemCommand},
    UiBuilder,
};

pub trait Cond {
    fn cond<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        Pos: Fn(&mut ChildBuilder) + Send + Sync + 'static,
        Neg: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        test: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self;
}

impl Cond for ChildBuilder<'_> {
    fn cond<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        Pos: Fn(&mut ChildBuilder) + Send + Sync + 'static,
        Neg: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self {
        // let test_sys = self.commands().register_system(test);
        self.spawn(EffectCell::new(CondEffect {
            state: false,
            test_fn: Some(test_fn),
            test_sys: None,
            pos,
            neg,
            marker: std::marker::PhantomData::<M>,
        }));
        self
    }
}

impl Cond for UiBuilder<'_> {
    fn cond<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        Pos: Fn(&mut ChildBuilder) + Send + Sync + 'static,
        Neg: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self {
        // let test_sys = self.commands().register_system(test);
        self.spawn(EffectCell::new(CondEffect {
            state: false,
            test_fn: Some(test_fn),
            test_sys: None,
            pos,
            neg,
            marker: std::marker::PhantomData::<M>,
        }));
        self
    }
}

impl Cond for WorldChildBuilder<'_> {
    fn cond<
        M: Send + Sync + 'static,
        TestFn: IntoSystem<(), bool, M> + Send + Sync + 'static,
        Pos: Fn(&mut ChildBuilder) + Send + Sync + 'static,
        Neg: Fn(&mut ChildBuilder) + Send + Sync + 'static,
    >(
        &mut self,
        test_fn: TestFn,
        pos: Pos,
        neg: Neg,
    ) -> &mut Self {
        // let test_sys = self.commands().register_system(test);
        self.spawn(EffectCell::new(CondEffect {
            state: false,
            test_fn: Some(test_fn),
            test_sys: None,
            pos,
            neg,
            marker: std::marker::PhantomData::<M>,
        }));
        self
    }
}

/// Conditional control-flow node.
struct CondEffect<
    M,
    TestFn: IntoSystem<(), bool, M> + 'static,
    Pos: Fn(&mut ChildBuilder),
    Neg: Fn(&mut ChildBuilder),
> {
    state: bool,
    test_fn: Option<TestFn>,
    test_sys: Option<SystemId<(), bool>>,
    pos: Pos,
    neg: Neg,
    marker: std::marker::PhantomData<M>,
}

impl<
        M,
        TestFn: IntoSystem<(), bool, M>,
        Pos: Fn(&mut ChildBuilder),
        Neg: Fn(&mut ChildBuilder),
    > AnyEffect for CondEffect<M, TestFn, Pos, Neg>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        let mut first = false;
        let test_sys = match self.test_sys {
            Some(sys) => sys,
            None => {
                let sys = world.register_system(self.test_fn.take().unwrap());
                self.test_sys = Some(sys);
                first = true;
                sys
            }
        };

        // Run the condition and see if the result changed.
        let test = world.run_system(test_sys);
        if let Ok(test) = test {
            if self.state != test || first {
                let mut entt = world.entity_mut(entity);
                entt.despawn_descendants();
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
        if let Some(test_sys) = self.test_sys {
            world.commands().queue(UnregisterSystemCommand(test_sys));
        }
    }
}
