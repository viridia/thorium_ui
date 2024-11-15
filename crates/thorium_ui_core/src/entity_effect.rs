use std::sync::{Arc, Mutex};

use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::effect_cell::{AnyEffect, EffectCell, UnregisterSystemCommand};

/// General dynamic effect which can be applied to an entity.
// pub struct CreateEntityEffect<
//     P,
//     M,
//     DepsFn: IntoSystem<(), P, M>,
//     EffectFn: Fn(P, &mut EntityWorldMut),
// > {
//     deps_fn: DepsFn,
//     effect_fn: EffectFn,
//     marker: std::marker::PhantomData<(M, P)>,
// }

// impl<
//         P: PartialEq + Clone + Send + Sync + 'static,
//         M: Send + Sync + 'static,
//         DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
//         EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
//     > CreateEntityEffect<P, M, DepsFn, EffectFn>
// {
//     pub fn new(deps_fn: DepsFn, effect_fn: EffectFn) -> Self {
//         Self {
//             deps_fn,
//             effect_fn,
//             marker: std::marker::PhantomData,
//         }
//     }
// }

// impl<
//         P: PartialEq + Clone + Send + Sync + 'static,
//         M: Send + Sync + 'static,
//         DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
//         EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
//     > Component for CreateEntityEffect<P, M, DepsFn, EffectFn>
// {
//     /// This is a sparse set component as it's only ever added and removed, never iterated over.
//     const STORAGE_TYPE: StorageType = StorageType::SparseSet;

//     fn register_component_hooks(hooks: &mut ComponentHooks) {
//         hooks.on_add(|mut world, entity, _| {
//             world
//                 .commands()
//                 .queue(WithEffectCommand::<P, M, DepsFn, EffectFn> {
//                     entity,
//                     marker: std::marker::PhantomData,
//                 });
//         });
//     }
// }

// pub struct WithEffectCommand<P, M, DepsFn, EffectFn> {
//     entity: Entity,
//     marker: std::marker::PhantomData<(P, M, DepsFn, EffectFn)>,
// }

// impl<
//         P: PartialEq + Clone + Send + Sync + 'static,
//         M: Send + Sync + 'static,
//         DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
//         EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
//     > Command for WithEffectCommand<P, M, DepsFn, EffectFn>
// {
//     fn apply(self, world: &mut bevy::prelude::World) {
//         let mut entt = world.entity_mut(self.entity);
//         let effect = entt
//             .take::<CreateEntityEffect<P, M, DepsFn, EffectFn>>()
//             .unwrap();
//         let deps_sys = world.register_system(effect.deps_fn);
//         world
//             .spawn(EffectCell::new(WithEffectAction {
//                 target: self.entity,
//                 deps: None,
//                 deps_sys,
//                 effect_fn: effect.effect_fn,
//                 marker: std::marker::PhantomData::<M>,
//             }))
//             .set_parent(self.entity);
//     }
// }

pub struct WithEffectAction<P, M, EffectFn: Fn(P, &mut EntityWorldMut)> {
    target: Entity,
    deps: Option<P>,
    deps_sys: SystemId<(), P>,
    effect_fn: EffectFn,
    marker: std::marker::PhantomData<M>,
}

impl<P: 'static + PartialEq + Clone, M, EffectFn: Fn(P, &mut EntityWorldMut)> AnyEffect
    for WithEffectAction<P, M, EffectFn>
{
    fn update(&mut self, world: &mut World, _entity: Entity) {
        // Run the dependencies and see if the result changed.
        let deps = world.run_system(self.deps_sys).ok();
        if deps.is_some() && deps != self.deps {
            self.deps = deps.clone();
            // Run the effect
            (self.effect_fn)(deps.unwrap(), &mut world.entity_mut(self.target));
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world
            .commands()
            .queue(UnregisterSystemCommand(self.deps_sys));
    }
}

pub trait EntityEffect {
    fn effect<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    >(
        &mut self,
        deps_fn: DepsFn,
        effect_fn: EffectFn,
    ) -> &mut Self;
}

impl EntityEffect for EntityCommands<'_> {
    fn effect<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    >(
        &mut self,
        deps_fn: DepsFn,
        effect_fn: EffectFn,
    ) -> &mut Self {
        let deps_sys = self.commands().register_system(deps_fn);
        let target = self.id();
        self.commands()
            .spawn(EffectCell(Arc::new(Mutex::new(WithEffectAction {
                target,
                deps: None,
                deps_sys,
                effect_fn,
                marker: std::marker::PhantomData::<M>,
            }))))
            .set_parent(target);
        self
    }
}
