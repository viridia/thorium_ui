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

unsafe impl<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    > Bundle for MutateDyn<P, M, DepsFn, EffectFn>
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
        todo!()
    }
}

impl<
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    > DynamicBundle for MutateDyn<P, M, DepsFn, EffectFn>
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
        P: PartialEq + Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        DepsFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        EffectFn: Fn(P, &mut EntityWorldMut) + Send + Sync + 'static,
    > BundleEffect for MutateDyn<P, M, DepsFn, EffectFn>
{
    fn apply(self, parent: &mut EntityWorldMut) {
        let target = parent.id();
        let world = unsafe { parent.world_mut() };
        let deps_sys = world.register_system(self.deps_fn);
        world.spawn((
            EffectCell::new(MutateDynEffect {
                target,
                deps: None,
                deps_sys,
                effect_fn: self.effect_fn,
                marker: std::marker::PhantomData::<M>,
            }),
            OwnedBy(target),
        ));
    }
}
