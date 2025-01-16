use std::marker::PhantomData;

use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
};

use crate::effect_cell::{AnyEffect, EffectCell};

/// A memoized computation.
#[derive(Copy, Clone, Debug)]
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

pub(crate) struct MemoEffect<P: PartialEq + Clone> {
    system: SystemId<(), P>,
}

#[derive(Component)]
pub(crate) struct MemoValue<P>(P);

impl<P: PartialEq + Clone + Send + Sync + 'static> AnyEffect for MemoEffect<P> {
    fn update(&mut self, world: &mut World, entity: Entity) {
        let value = world.run_system(self.system).unwrap();
        let Ok(mut entt) = world.get_entity_mut(entity) else {
            return;
        };
        let mut value_ref = entt.get_mut::<MemoValue<P>>().unwrap();
        if value_ref.0 != value {
            value_ref.0 = value;
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.system);
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

impl CreateMemo for ChildSpawnerCommands<'_> {
    fn create_memo<
        M: Send + Sync + 'static,
        P: PartialEq + Clone + Send + Sync + 'static,
        I: IntoSystem<(), P, M> + Send + Sync + 'static,
    >(
        &mut self,
        factory: I,
        default_value: P,
    ) -> Memo<P> {
        let mut entity = self.spawn_empty();
        let system = entity.commands().register_system(factory);
        entity.insert((
            EffectCell::new(MemoEffect { system }),
            MemoValue(default_value),
        ));
        Memo {
            entity: entity.id(),
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
        let mut entity = self.spawn_empty();
        let system = entity.commands().register_system(factory);
        entity.insert((
            EffectCell::new(MemoEffect { system }),
            MemoValue(default_value),
        ));
        Memo {
            entity: entity.id(),
            marker: PhantomData,
        }
    }
}

/// Methods for reading a memoized computation.
pub trait ReadMemo {
    /// Reads the memoized value from the given memo.
    fn read_memo<P: Clone + Send + Sync + 'static>(&self, memo: Memo<P>) -> P;

    /// Read the value of a mutable variable using a mapping function. This adds any dependencies of
    /// the derived signal to the current tracking scope.
    fn read_memo_map<P, U, F: Fn(&P) -> U>(&self, derived: &Memo<P>, f: F) -> U
    where
        P: Send + Sync + 'static;
}

impl ReadMemo for World {
    fn read_memo<P: Clone + Send + Sync + 'static>(&self, memo: Memo<P>) -> P {
        self.entity(memo.entity)
            .get::<MemoValue<P>>()
            .unwrap()
            .0
            .clone()
    }

    fn read_memo_map<P, U, F: Fn(&P) -> U>(&self, memo: &Memo<P>, f: F) -> U
    where
        P: Send + Sync + 'static,
    {
        self.entity(memo.entity)
            .get::<MemoValue<P>>()
            .map(|value| f(&value.0))
            .unwrap()
    }
}

impl ReadMemo for DeferredWorld<'_> {
    fn read_memo<P: Clone + Send + Sync + 'static>(&self, memo: Memo<P>) -> P {
        self.entity(memo.entity)
            .get::<MemoValue<P>>()
            .unwrap()
            .0
            .clone()
    }

    fn read_memo_map<P, U, F: Fn(&P) -> U>(&self, memo: &Memo<P>, f: F) -> U
    where
        P: Send + Sync + 'static,
    {
        self.entity(memo.entity)
            .get::<MemoValue<P>>()
            .map(|value| f(&value.0))
            .unwrap()
    }
}
