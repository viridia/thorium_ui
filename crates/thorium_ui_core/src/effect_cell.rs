use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

/// Component which holds a type-erased entity effect. An effect represents some dynamic mutation
/// of the entity's state.
/// Note: If Bevy had trait queries, we wouldn't the Arc/Mutex.
#[derive(Component, Clone)]
// #[require(GhostNode)]
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

fn on_add_effect(mut world: DeferredWorld, entity: Entity, _cid: ComponentId) {
    world.commands().queue(RunEffectNow(entity));
}

fn on_remove_effect(mut world: DeferredWorld, entity: Entity, _cid: ComponentId) {
    let cell = world.get_mut::<EffectCell>(entity).unwrap();
    let comp = cell.effect.clone();
    comp.lock().unwrap().cleanup(&mut world, entity);
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

pub trait ConstructEffect {
    fn construct(self, parent: &mut EntityCommands<'_>);
}

/// `EffectTuple` - a variable-length tuple of effects.
pub trait EffectTuple: Sync + Send {
    /// Method to construct the effect on the target entity.
    fn apply(self, commands: &mut EntityCommands);
}

/// Empty effect tuple.
impl EffectTuple for () {
    fn apply(self, _ctx: &mut EntityCommands) {}
}

impl<E: ConstructEffect + Send + Sync + 'static> EffectTuple for E {
    fn apply(self, ctx: &mut EntityCommands) {
        self.construct(ctx);
    }
}

macro_rules! impl_effect_tuple {
    ( $($style: ident, $idx: tt);+ ) => {
        impl<$(
            $style: EffectTuple + 'static,
        )+> EffectTuple for ( $( $style, )* ) {
            fn apply(self, builder: &mut EntityCommands) {
                $( self.$idx.apply(builder); )*
            }
        }
    };
}

impl_effect_tuple!(E0, 0);
impl_effect_tuple!(E0, 0; E1, 1);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14);
impl_effect_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14; E15, 15);

pub trait BuildEffects {
    fn effects(&mut self, effect_tuple: impl EffectTuple) -> &mut Self;
}

impl BuildEffects for EntityCommands<'_> {
    fn effects(&mut self, effect_tuple: impl EffectTuple) -> &mut Self {
        effect_tuple.apply(self);
        self
    }
}
