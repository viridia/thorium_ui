use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui::experimental::GhostNode,
};

use crate::{
    effect_cell::{AnyEffect, EffectCell},
    owner::Owned,
};

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
