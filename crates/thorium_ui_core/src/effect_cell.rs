use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

/// Component which holds a type-erased entity effect. An effect represents some dynamic mutation
/// of the entity's state.
/// Note: If Bevy had trait queries, we wouldn't the Arc/Mutex.
#[derive(Component, Clone)]
#[component(on_add = on_add_effect, on_remove = on_remove_effect)]
pub struct EffectCell {
    pub(crate) effect: Arc<Mutex<dyn AnyEffect + 'static + Sync + Send>>,
    order: usize,
}

static EFFECT_ORDER: AtomicUsize = AtomicUsize::new(0);

impl EffectCell {
    pub fn new<E: AnyEffect + 'static + Sync + Send>(effect: E) -> Self {
        Self {
            effect: Arc::new(Mutex::new(effect)),
            order: EFFECT_ORDER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

pub(crate) trait AnyEffect {
    fn update(&mut self, world: &mut World, entity: Entity);
    fn cleanup(&self, world: &mut DeferredWorld, entity: Entity);
}

fn on_add_effect(mut world: DeferredWorld, context: HookContext) {
    world.commands().queue(RunEffectNow(context.entity));
}

fn on_remove_effect(mut world: DeferredWorld, context: HookContext) {
    let cell = world.get_mut::<EffectCell>(context.entity).unwrap();
    let comp = cell.effect.clone();
    comp.lock().unwrap().cleanup(&mut world, context.entity);
}

pub(crate) fn update_effects(world: &mut World) {
    let mut query = world.query::<(Entity, &EffectCell)>();
    let mut effects = query
        .iter(world)
        .map(|(entity, eff)| (entity, eff.clone()))
        .collect::<Vec<_>>();
    // Sort effects by creation order
    effects.sort_by(|a, b| a.1.order.cmp(&b.1.order));
    for (entity, eff) in effects {
        eff.effect.lock().unwrap().update(world, entity);
    }
}

struct RunEffectNow(pub Entity);

impl Command for RunEffectNow {
    fn apply(self, world: &mut World) {
        let cell = world.get_mut::<EffectCell>(self.0).unwrap();
        let effect = cell.effect.clone();
        effect.lock().unwrap().update(world, self.0);
    }
}
