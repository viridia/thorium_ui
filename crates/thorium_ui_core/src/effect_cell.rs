use std::sync::{Arc, Mutex};

use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui::experimental::GhostNode,
};

/// Component which holds a type-erased entity effect. An effect represents some dynamic mutation
/// of the entity's state.
#[derive(Component)]
#[require(GhostNode)]
pub struct EffectCell(pub(crate) Arc<Mutex<dyn AnyEffect + 'static + Sync + Send>>);

impl EffectCell {
    pub fn new<E: AnyEffect + 'static + Sync + Send>(effect: E) -> Self {
        Self(Arc::new(Mutex::new(effect)))
    }
}

pub(crate) trait AnyEffect {
    fn update(&mut self, world: &mut World, entity: Entity);
    fn cleanup(&self, world: &mut DeferredWorld, entity: Entity);
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
        app.world_mut()
            .register_component_hooks::<EffectCell>()
            .on_remove(|mut world, entity, _cond| {
                let cell = world.get_mut::<EffectCell>(entity).unwrap();
                let comp = cell.0.clone();
                comp.lock().unwrap().cleanup(&mut world, entity);
            });
    }
}

pub fn update_effects(world: &mut World) {
    let mut query = world.query::<(Entity, &EffectCell)>();
    let effects = query
        .iter(world)
        .map(|(entity, eff)| (entity, eff.0.clone()))
        .collect::<Vec<_>>();
    for (entity, eff) in effects {
        eff.lock().unwrap().update(world, entity);
    }
}

pub(crate) struct UnregisterSystemCommand<I: SystemInput, O>(pub(crate) SystemId<I, O>);

impl<I: SystemInput + 'static, O: 'static> Command for UnregisterSystemCommand<I, O> {
    fn apply(self, world: &mut World) {
        world.remove_system(self.0).unwrap();
    }
}
