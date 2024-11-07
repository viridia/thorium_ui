#![allow(clippy::type_complexity)]
use bevy::{ecs::system::SystemId, prelude::*};

use crate::effect_cell::{AnyEffect, EffectCell, UnregisterSystemCommand};

pub trait Switch {
    fn switch<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        CF: Fn(&mut CaseBuilder<P>),
    >(
        &mut self,
        value_fn: ValueFn,
        cases_fn: CF,
    ) -> &mut Self;
}

impl<'w> Switch for ChildBuilder<'w> {
    fn switch<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        CF: Fn(&mut CaseBuilder<P>),
    >(
        &mut self,
        value_fn: ValueFn,
        cases_fn: CF,
    ) -> &mut Self {
        let mut cases: Vec<(P, Box<dyn Fn(&mut ChildBuilder) + Send + Sync>)> = Vec::new();
        let mut fallback: Option<Box<dyn Fn(&mut ChildBuilder) + Send + Sync>> = None;

        let mut case_builder = CaseBuilder {
            cases: &mut cases,
            fallback: &mut fallback,
        };
        cases_fn(&mut case_builder);
        self.spawn(EffectCell::new(SwitchEffect {
            cases,
            fallback,
            value_fn: Some(value_fn),
            value_sys: None,
            switch_index: usize::MAX - 1, // Means no case selected yet.
            marker: std::marker::PhantomData,
        }));
        self
    }
}

impl<'w> Switch for WorldChildBuilder<'w> {
    fn switch<
        M: Send + Sync + 'static,
        P: PartialEq + Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
        CF: Fn(&mut CaseBuilder<P>),
    >(
        &mut self,
        value_fn: ValueFn,
        cases_fn: CF,
    ) -> &mut Self {
        let mut cases: Vec<(P, Box<dyn Fn(&mut ChildBuilder) + Send + Sync>)> = Vec::new();
        let mut fallback: Option<Box<dyn Fn(&mut ChildBuilder) + Send + Sync>> = None;

        let mut case_builder = CaseBuilder {
            cases: &mut cases,
            fallback: &mut fallback,
        };
        cases_fn(&mut case_builder);
        self.spawn(EffectCell::new(SwitchEffect {
            cases,
            fallback,
            value_fn: Some(value_fn),
            value_sys: None,
            switch_index: usize::MAX - 1, // Means no case selected yet.
            marker: std::marker::PhantomData,
        }));
        self
    }
}

pub struct CaseBuilder<'a, Value: Send + Sync> {
    cases: &'a mut Vec<(Value, Box<dyn Fn(&mut ChildBuilder) + Send + Sync>)>,
    fallback: &'a mut Option<Box<dyn Fn(&mut ChildBuilder) + Send + Sync>>,
}

impl<'a, Value: Send + Sync> CaseBuilder<'a, Value> {
    pub fn case<CF: Send + Sync + 'static + Fn(&mut ChildBuilder)>(
        &mut self,
        value: Value,
        case_fn: CF,
    ) -> &mut Self {
        self.cases.push((value, Box::new(case_fn)));
        self
    }

    pub fn fallback<FF: Send + Sync + 'static + Fn(&mut ChildBuilder)>(
        &mut self,
        fallback_fn: FF,
    ) -> &mut Self {
        *self.fallback = Some(Box::new(fallback_fn));
        self
    }
}

/// Conditional control-flow node that implements a C-like "switch" statement.
struct SwitchEffect<P, M, ValueFn: IntoSystem<(), P, M>> {
    switch_index: usize,
    value_fn: Option<ValueFn>,
    value_sys: Option<SystemId<(), P>>,
    cases: Vec<(P, Box<dyn Fn(&mut ChildBuilder) + Send + Sync>)>,
    fallback: Option<Box<dyn Fn(&mut ChildBuilder) + Send + Sync>>,
    marker: std::marker::PhantomData<M>,
}

impl<
        P: PartialEq + Send + Sync + 'static,
        M: Send + Sync + 'static,
        ValueFn: IntoSystem<(), P, M> + Send + Sync + 'static,
    > SwitchEffect<P, M, ValueFn>
{
    /// Adds a new switch case.
    #[allow(dead_code)]
    pub fn case<F: Fn(&mut ChildBuilder) + Send + Sync + 'static>(
        mut self,
        value: P,
        case: F,
    ) -> Self {
        self.cases.push((value, Box::new(case)));
        self
    }

    /// Sets the fallback case.
    #[allow(dead_code)]
    pub fn fallback<F: Fn(&mut ChildBuilder) + Send + Sync + 'static>(
        mut self,
        fallback: F,
    ) -> Self {
        self.fallback = Some(Box::new(fallback));
        self
    }
}

impl<P: PartialEq + 'static, M, ValueFn: IntoSystem<(), P, M> + 'static> AnyEffect
    for SwitchEffect<P, M, ValueFn>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        // The first time we run, we need to register the one-shot system.
        let mut first = false;
        if let Some(test) = self.value_fn.take() {
            let value_sys = world.register_system(test);
            self.value_sys = Some(value_sys);
            first = true;
        }

        // Run the condition and see if the result changed.
        if let Some(test_id) = self.value_sys {
            let value = world.run_system(test_id);

            if let Ok(value) = value {
                let index = self
                    .cases
                    .iter()
                    .enumerate()
                    .find_map(|(i, f)| if f.0 == value { Some(i) } else { None })
                    .unwrap_or(usize::MAX);

                if self.switch_index != index || first {
                    self.switch_index = index;
                    let mut commands = world.commands();
                    let mut entt = commands.entity(entity);
                    entt.despawn_descendants();
                    if index < self.cases.len() {
                        entt.with_children(|builder| {
                            (self.cases[index].1)(builder);
                        });
                    } else if let Some(fallback) = self.fallback.as_mut() {
                        entt.with_children(|builder| {
                            (fallback)(builder);
                        });
                    };
                }
            }
        }
    }

    fn cleanup(&self, world: &mut bevy::ecs::world::DeferredWorld, _entity: Entity) {
        if let Some(test_id) = self.value_sys {
            world.commands().queue(UnregisterSystemCommand(test_id));
        }
    }
}
