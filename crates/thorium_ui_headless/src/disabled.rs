use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::{Component, Entity, World},
};

/// A marker component to indicate that a widget is disabled and should be "grayed out".
#[derive(Component, Debug, Clone, Copy)]
#[component(on_add = on_add_disabled, on_remove = on_remove_disabled)]
pub struct InteractionDisabled;

// Hook to set the a11y "checked" state when the checkbox is added.
fn on_add_disabled(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_disabled();
    }
}

// Hook to set the a11y "checked" state when the checkbox is added.
fn on_remove_disabled(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.clear_disabled();
    }
}

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
