use core::slice;

use bevy::prelude::*;

/// A component that represents the owner of an entity. Ownership only determines lifetime,
/// such that the owned entity will be despawned when its owner is despawned. It does not imply
/// any other kind of semantic connection between the two entities.
#[derive(Component, Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Component, PartialEq, Debug, FromWorld)]
#[relationship(relationship_sources = Owned)]
pub struct OwnedBy(pub Entity);

impl OwnedBy {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl Default for OwnedBy {
    fn default() -> Self {
        OwnedBy(Entity::PLACEHOLDER)
    }
}

#[derive(Component, Default, Reflect)]
#[relationship_sources(relationship = OwnedBy, despawn_descendants)]
#[reflect(Component)]
pub struct Owned(Vec<Entity>);

impl<'a> IntoIterator for &'a Owned {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl core::ops::Deref for Owned {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
