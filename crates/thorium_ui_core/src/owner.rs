use core::slice;

use bevy::{
    ecs::relationship::{Relationship, RelationshipSources},
    prelude::*,
};

/// A component that represents the owner of an entity. Ownership only determines lifetime,
/// such that the owned entity will be despawned when its owner is despawned. It does not imply
/// any other kind of semantic connection between the two entities.
#[derive(Relationship, Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Component, PartialEq, Debug, FromWorld)]
#[relationship(relationship_sources = OwnedBy)]
pub struct Owner(pub Entity);

impl Owner {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl Default for Owner {
    fn default() -> Self {
        Owner(Entity::PLACEHOLDER)
    }
}

#[derive(RelationshipSources, Default, Reflect)]
#[relationship_sources(relationship = Owner, despawn_descendants)]
#[reflect(Component)]
pub struct OwnedBy(Vec<Entity>);

impl<'a> IntoIterator for &'a OwnedBy {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl core::ops::Deref for OwnedBy {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
