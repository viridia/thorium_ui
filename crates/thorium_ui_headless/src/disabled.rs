use bevy::{
    ecs::world::DeferredWorld,
    prelude::{Component, Entity, World},
};

/// A marker component to indicate that a widget is disabled and should be "grayed out".
#[derive(Component, Debug, Clone, Copy)]
pub struct InteractionDisabled;

/// Trait which defines a method to check if an entity is disabled.
pub trait IsInteractionDisabled {
    /// Returns true if the given entity is disabled.
    fn is_interaction_disabled(&self, entity: Entity) -> bool;
}

impl IsInteractionDisabled for DeferredWorld<'_> {
    fn is_interaction_disabled(&self, entity: Entity) -> bool {
        self.get::<InteractionDisabled>(entity).is_some()
    }
}

impl IsInteractionDisabled for World {
    fn is_interaction_disabled(&self, entity: Entity) -> bool {
        self.get::<InteractionDisabled>(entity).is_some()
    }
}
