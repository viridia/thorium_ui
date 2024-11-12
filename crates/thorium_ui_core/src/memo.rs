use std::marker::PhantomData;

use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::effect_cell::{AnyEffect, EffectCell, UnregisterSystemCommand};

/// A memoized computation.
#[derive(Copy, Clone)]
pub struct Memo<P> {
    entity: Entity,
    marker: PhantomData<P>,
}

impl<P> Memo<P> {
    /// Returns the entity associated with this memo.
    pub fn entity(&self) -> Entity {
        self.entity
    }
}

pub(crate) struct MemoEffect<M, P: PartialEq + Clone, I: IntoSystem<(), P, M>> {
    factory: Option<I>,
    system: Option<SystemId<(), P>>,
    marker: PhantomData<M>,
}

#[derive(Component)]
pub(crate) struct MemoValue<P>(P);

impl<M, P: PartialEq + Clone + Send + Sync + 'static, I: IntoSystem<(), P, M> + 'static> AnyEffect
    for MemoEffect<M, P, I>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        let system = match self.system {
            Some(sys) => sys,
            None => {
                let sys = world.register_system(self.factory.take().unwrap());
                self.system = Some(sys);
                sys
            }
        };

        let value = world.run_system(system).unwrap();
        let mut entt = world.entity_mut(entity);
        let mut value_ref = entt.get_mut::<MemoValue<P>>().unwrap();
        if value_ref.0 != value {
            value_ref.0 = value;
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        if let Some(system) = self.system {
            world.commands().queue(UnregisterSystemCommand(system));
        }
    }
}

/// Methods for creating a memoized computation.
pub trait CreateMemo {
    /// Registers a scoped one-shot system, with no input, that will be removed when the parent
    /// entity is despawned.
    fn create_memo<
        M: Send + Sync + 'static,
        P: PartialEq + Clone + Send + Sync + 'static,
        I: IntoSystem<(), P, M> + Send + Sync + 'static,
    >(
        &mut self,
        factory: I,
        default_value: P,
    ) -> Memo<P>;
}

impl CreateMemo for ChildBuilder<'_> {
    fn create_memo<
        M: Send + Sync + 'static,
        P: PartialEq + Clone + Send + Sync + 'static,
        I: IntoSystem<(), P, M> + Send + Sync + 'static,
    >(
        &mut self,
        factory: I,
        default_value: P,
    ) -> Memo<P> {
        let entity = self
            .spawn((
                EffectCell::new(MemoEffect {
                    factory: Some(factory),
                    system: None,
                    marker: PhantomData,
                }),
                MemoValue(default_value),
            ))
            .id();
        Memo {
            entity,
            marker: PhantomData,
        }
    }
}

impl CreateMemo for Commands<'_, '_> {
    fn create_memo<
        M: Send + Sync + 'static,
        P: PartialEq + Clone + Send + Sync + 'static,
        I: IntoSystem<(), P, M> + Send + Sync + 'static,
    >(
        &mut self,
        factory: I,
        default_value: P,
    ) -> Memo<P> {
        let entity = self
            .spawn((
                EffectCell::new(MemoEffect {
                    factory: Some(factory),
                    system: None,
                    marker: PhantomData,
                }),
                MemoValue(default_value),
            ))
            .id();
        Memo {
            entity,
            marker: PhantomData,
        }
    }
}

/// Methods for reading a memoized computation.
pub trait ReadMemo {
    /// Reads the memoized value from the given memo.
    fn read_memo<P: PartialEq + Clone + Send + Sync + 'static>(&mut self, memo: Memo<P>) -> P;
}

impl ReadMemo for World {
    fn read_memo<P: PartialEq + Clone + Send + Sync + 'static>(&mut self, memo: Memo<P>) -> P {
        self.entity(memo.entity)
            .get::<MemoValue<P>>()
            .unwrap()
            .0
            .clone()
    }
}

impl ReadMemo for DeferredWorld<'_> {
    fn read_memo<P: PartialEq + Clone + Send + Sync + 'static>(&mut self, memo: Memo<P>) -> P {
        self.entity(memo.entity)
            .get::<MemoValue<P>>()
            .unwrap()
            .0
            .clone()
    }
}
