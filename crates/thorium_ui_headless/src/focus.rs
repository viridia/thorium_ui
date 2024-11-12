use bevy::{
    ecs::world::DeferredWorld, input::keyboard::KeyboardInput, prelude::*, window::PrimaryWindow,
};

/// Resource representing which entity has keyboard focus, if any.
#[derive(Clone, Debug, Resource)]
pub struct KeyboardFocus(pub Option<Entity>);

/// Helper functions for [`World`] and [`DeferredWorld`] to set and clear keyboard focus.
pub trait SetKeyboardFocus {
    /// Set keyboard focus to the given entity.
    fn set_keyboard_focus(&mut self, entity: Entity);
    /// Clear keyboard focus.
    fn clear_keyboard_focus(&mut self);
}

impl SetKeyboardFocus for World {
    fn set_keyboard_focus(&mut self, entity: Entity) {
        if let Some(mut focus) = self.get_resource_mut::<KeyboardFocus>() {
            focus.0 = Some(entity);
        }
    }

    fn clear_keyboard_focus(&mut self) {
        if let Some(mut focus) = self.get_resource_mut::<KeyboardFocus>() {
            focus.0 = None;
        }
    }
}

impl<'w> SetKeyboardFocus for DeferredWorld<'w> {
    fn set_keyboard_focus(&mut self, entity: Entity) {
        if let Some(mut focus) = self.get_resource_mut::<KeyboardFocus>() {
            focus.0 = Some(entity);
        }
    }

    fn clear_keyboard_focus(&mut self) {
        if let Some(mut focus) = self.get_resource_mut::<KeyboardFocus>() {
            focus.0 = None;
        }
    }
}

/// Command to set keyboard focus to the given entity.
pub struct SetFocusCommand(Option<Entity>);

impl Command for SetFocusCommand {
    fn apply(self, world: &mut World) {
        if let Some(mut focus) = world.get_resource_mut::<KeyboardFocus>() {
            focus.0 = self.0;
        }
    }
}

/// A bubble-able event for keyboard input. This event is normally dispatched to the current
/// keyboard focus entity, if any. If no entity has keyboard focus, then the event is dispatched to
/// the main window.
#[derive(Clone, Debug, Component)]
pub struct FocusKeyboardInput(pub KeyboardInput);

impl Event for FocusKeyboardInput {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}

/// Plugin which registers the system for dispatching keyboard events based on focus and
/// hover state.
pub struct InputDispatchPlugin;

impl Plugin for InputDispatchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyboardFocus(None))
            .add_systems(Update, dispatch_keyboard_input);
    }
}

/// System whcich dispatches keyboard input events to the focused entity, or to the primary window
/// if no entity has focus.
fn dispatch_keyboard_input(
    mut key_events: EventReader<KeyboardInput>,
    focus: Res<KeyboardFocus>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    // If an element has keyboard focus, then dispatch the key event to that element.
    if let Some(focus_elt) = focus.0 {
        for ev in key_events.read() {
            commands.trigger_targets(FocusKeyboardInput(ev.clone()), focus_elt);
        }
    } else {
        // If no element has keyboard focus, then dispatch the key event to the primary window.
        // There should be only one primary window.
        if let Ok(window) = windows.get_single() {
            for ev in key_events.read() {
                commands.trigger_targets(FocusKeyboardInput(ev.clone()), window);
            }
        }
    }
}
