#![allow(clippy::type_complexity)]
use bevy::{
    ecs::{
        bundle::{BundleEffect, DynamicBundle},
        system::SystemId,
    },
    prelude::*,
};

use crate::{
    dyn_children::Fragment,
    effect_cell::{AnyEffect, EffectCell},
    owner::Owned,
    Computations, DynChildren, SpawnableListGen,
};

pub struct CaseBuilder<'a, Value: Send + Sync> {
    cases: &'a mut Vec<(Value, Box<dyn SpawnableListGen + Send + Sync>)>,
    fallback: &'a mut Option<Box<dyn SpawnableListGen + Send + Sync>>,
}

impl<Value: Send + Sync> CaseBuilder<'_, Value> {
    pub fn case<CF: Send + Sync + 'static + SpawnableListGen>(
        &mut self,
        value: Value,
        case_fn: CF,
    ) -> &mut Self {
        self.cases.push((value, Box::new(case_fn)));
        self
    }

    pub fn fallback<FF: Send + Sync + 'static + SpawnableListGen>(
        &mut self,
        fallback_fn: FF,
    ) -> &mut Self {
        *self.fallback = Some(Box::new(fallback_fn));
        self
    }
}

/// Conditional control-flow node that implements a C-like "switch" statement.
struct SwitchEffect<P> {
    switch_index: usize,
    value_sys: SystemId<(), P>,
    cases: Vec<(P, Box<dyn SpawnableListGen + Send + Sync>)>,
    fallback: Option<Box<dyn SpawnableListGen + Send + Sync>>,
}

impl<P: PartialEq + Send + Sync + 'static> SwitchEffect<P> {
    /// Adds a new switch case.
    #[allow(dead_code)]
    pub fn case<F: SpawnableListGen + Send + Sync + 'static>(mut self, value: P, case: F) -> Self {
        self.cases.push((value, Box::new(case)));
        self
    }

    /// Sets the fallback case.
    #[allow(dead_code)]
    pub fn fallback<F: SpawnableListGen + Send + Sync + 'static>(mut self, fallback: F) -> Self {
        self.fallback = Some(Box::new(fallback));
        self
    }
}

impl<P: PartialEq + Send + Sync + 'static> AnyEffect for SwitchEffect<P> {
    fn update(&mut self, world: &mut World, entity: Entity) {
        // Run the condition and see if the result changed.
        let value = world.run_system(self.value_sys);
        if let Ok(value) = value {
            let index = self
                .cases
                .iter()
                .enumerate()
                .find_map(|(i, f)| if f.0 == value { Some(i) } else { None })
                .unwrap_or(usize::MAX);

            if self.switch_index != index {
                self.switch_index = index;
                let mut commands = world.commands();
                let mut entt = commands.entity(entity);
                entt.despawn_related::<DynChildren>();
                entt.despawn_related::<Computations>();
                entt.despawn_related::<Owned>();
                if index < self.cases.len() {
                    self.cases[index].1.spawn(world, entity);
                } else if let Some(fallback) = self.fallback.as_mut() {
                    fallback.spawn(world, entity);
                };
            }
        }
    }

    fn cleanup(&self, world: &mut bevy::ecs::world::DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.value_sys);
    }
}

pub struct Switch<
    M: Send + Sync + 'static,
    P: PartialEq + Send + Sync + 'static,
    ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
> {
    value_fn: ValueFn,
    cases: Vec<(P, Box<dyn SpawnableListGen + Send + Sync>)>,
    fallback: Option<Box<dyn SpawnableListGen + Send + Sync>>,
    marker: std::marker::PhantomData<M>,
}

impl<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
    > Switch<M, P, ValueFn>
{
    pub fn new<CF: Fn(&mut CaseBuilder<P>)>(value_fn: ValueFn, cases_fn: CF) -> Self {
        let mut cases: Vec<(P, Box<dyn SpawnableListGen + Send + Sync>)> = Vec::new();
        let mut fallback: Option<Box<dyn SpawnableListGen + Send + Sync>> = None;

        let mut case_builder = CaseBuilder {
            cases: &mut cases,
            fallback: &mut fallback,
        };
        cases_fn(&mut case_builder);

        Self {
            value_fn,
            cases,
            fallback,
            marker: std::marker::PhantomData,
        }
    }
}

impl<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
    > BundleEffect for Switch<M, P, ValueFn>
{
    fn apply(self, entity: &mut EntityWorldMut) {
        let value_sys = entity.world_scope(|world| world.register_system(self.value_fn));
        entity.insert((
            EffectCell::new(SwitchEffect {
                cases: self.cases,
                fallback: self.fallback,
                value_sys,
                switch_index: usize::MAX - 1, // Means no case selected yet.
            }),
            // GhostNode::default(),
            Fragment,
        ));
    }
}

impl<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
    > DynamicBundle for Switch<M, P, ValueFn>
{
    type Effect = Self;

    fn get_components(
        self,
        _func: &mut impl FnMut(bevy::ecs::component::StorageType, bevy::ptr::OwningPtr<'_>),
    ) -> Self::Effect {
        self
    }
}

unsafe impl<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
    > Bundle for Switch<M, P, ValueFn>
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
