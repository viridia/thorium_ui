use bevy::{
    ecs::{relationship::RelatedSpawnerCommands, system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::effect_cell::{AnyEffect, EffectCell};

pub struct EffectHookAction<P, M, EffectFn: Fn(P, DeferredWorld)> {
    deps: Option<P>,
    deps_sys: SystemId<(), P>,
    effect_fn: EffectFn,
    marker: std::marker::PhantomData<M>,
}

impl<P: 'static + PartialEq + Send + Sync + Clone, M, EffectFn: Fn(P, DeferredWorld)> AnyEffect
    for EffectHookAction<P, M, EffectFn>
{
    fn update(&mut self, world: &mut World, _entity: Entity) {
        // Run the dependencies and see if the result changed.
        let deps = world.run_system(self.deps_sys).ok();
        if deps.is_some() && deps != self.deps {
            self.deps = deps.clone();
            // Run the effect
            (self.effect_fn)(deps.unwrap(), DeferredWorld::from(world));
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.deps_sys);
    }
}

pub trait CreateHookEffect {
    fn create_effect<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + 'static,
        EffectFn: Fn(P, DeferredWorld) + Send + Sync + 'static,
    >(
        &mut self,
        deps_fn: DepsFn,
        effect_fn: EffectFn,
    ) -> &mut Self;
}

impl CreateHookEffect for RelatedSpawnerCommands<'_, Parent> {
    fn create_effect<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + 'static,
        EffectFn: Fn(P, DeferredWorld) + Send + Sync + 'static,
    >(
        &mut self,
        deps_fn: DepsFn,
        effect_fn: EffectFn,
    ) -> &mut Self {
        let mut ent = self.spawn_empty();
        let deps_sys = ent.commands().register_system(deps_fn);
        ent.insert(EffectCell::new(EffectHookAction {
            deps: None,
            deps_sys,
            effect_fn,
            marker: std::marker::PhantomData::<M>,
        }));
        self
    }
}
