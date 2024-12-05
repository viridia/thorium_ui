#![allow(clippy::type_complexity)]
use bevy::{ecs::system::SystemId, prelude::*};

use crate::effect_cell::{AnyEffect, EffectCell};

pub trait CreateSwitch {
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

impl<'w> CreateSwitch for ChildBuilder<'w> {
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
        let mut ent = self.spawn_empty();
        let value_sys = ent.commands().register_system(value_fn);
        ent.insert(EffectCell::new(SwitchEffect {
            cases,
            fallback,
            value_sys,
            switch_index: usize::MAX - 1, // Means no case selected yet.
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
struct SwitchEffect<P> {
    switch_index: usize,
    value_sys: SystemId<(), P>,
    cases: Vec<(P, Box<dyn Fn(&mut ChildBuilder) + Send + Sync>)>,
    fallback: Option<Box<dyn Fn(&mut ChildBuilder) + Send + Sync>>,
}

impl<P: PartialEq + Send + Sync + 'static> SwitchEffect<P> {
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

    fn cleanup(&self, world: &mut bevy::ecs::world::DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.value_sys);
    }
}
