use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    input::ButtonState,
    input_focus::{FocusKeyboardInput, SetInputFocus},
    prelude::*,
};

/// A "brrier" is a backdrop element, one that covers the entire screen, blocks click events
/// from reaching elements behind it, and can be used to close a dialog or menu.
#[derive(Component, Debug)]
pub struct CoreBarrier {
    pub on_close: Option<SystemId>,
}

pub(crate) fn barrier_on_key_input(
    mut trigger: Trigger<FocusKeyboardInput>,
    q_state: Query<&CoreBarrier>,
    mut commands: Commands,
) {
    if let Ok(bstate) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Escape)
        {
            if let Some(on_close) = bstate.on_close {
                trigger.propagate(false);
                commands.run_system(on_close);
            }
        }
    }
}

pub(crate) fn barrier_on_pointer_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    q_state: Query<&CoreBarrier>,
    mut world: DeferredWorld,
    mut commands: Commands,
) {
    let entity_id = trigger.target();
    if let Ok(bstate) = q_state.get(entity_id) {
        world.set_input_focus(entity_id);
        trigger.propagate(false);
        if let Some(on_close) = bstate.on_close {
            commands.run_system(on_close);
        }
    }
}
