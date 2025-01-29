use bevy::prelude::*;

/// A relationship component used to associate a computation with its owner. A "computation" is
/// a formula or calculation that continuously updates the state of an entity based on the output
/// of some expression. The computation is owned by the entity that it updates, and ceases to exist
/// when the owner is despawned.
#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship(relationship_target = Computations)]
pub struct ComputationOf(pub Entity);

impl ComputationOf {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl Default for ComputationOf {
    fn default() -> Self {
        ComputationOf(Entity::PLACEHOLDER)
    }
}

/// A collection of computations. See [`ComputationOf`].
#[derive(Component, Default)]
#[relationship_target(relationship = ComputationOf, despawn_descendants)]
pub struct Computations(Vec<Entity>);

impl core::ops::Deref for Computations {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! computations {
    [$($child:expr),*$(,)?] => {
       $crate::Computations::spawn(($(Spawn($child)),*))
    };
}
