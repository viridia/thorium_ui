use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::{
    effect_cell::{AnyEffect, EffectCell},
    Attachment,
};

pub struct MutateDynEffect<P, M, EffectFn: Fn(P, &mut EntityWorldMut)> {
    target: Entity,
    deps: Option<P>,
    deps_sys: SystemId<(), P>,
    effect_fn: EffectFn,
    marker: std::marker::PhantomData<M>,
}

impl<P: 'static + PartialEq + Send + Sync + Clone, M, EffectFn: Fn(P, &mut EntityWorldMut)>
    AnyEffect for MutateDynEffect<P, M, EffectFn>
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
        world.commands().unregister_system(self.deps_sys);
    }
}

pub struct MutateDyn<
    P: PartialEq + Clone + Send + Sync + 'static,
    M: Send + Sync + 'static,
    DepsFn: IntoSystem<(), P, M> + 'static,
    EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
> {
    deps_fn: DepsFn,
    effect_fn: EffectFn,
    marker: std::marker::PhantomData<(P, M)>,
}

impl<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    > MutateDyn<P, M, DepsFn, EffectFn>
{
    pub fn new(deps_fn: DepsFn, effect_fn: EffectFn) -> Self {
        Self {
            deps_fn,
            effect_fn,
            marker: std::marker::PhantomData,
        }
    }
}
impl<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    > Attachment for MutateDyn<P, M, DepsFn, EffectFn>
{
    fn apply(self, parent: &mut EntityCommands<'_>) {
        let deps_sys = parent.commands().register_system(self.deps_fn);
        let target = parent.id();
        parent.commands().spawn(EffectCell::new(MutateDynEffect {
            target,
            deps: None,
            deps_sys,
            effect_fn: self.effect_fn,
            marker: std::marker::PhantomData::<M>,
        }));
    }
}
