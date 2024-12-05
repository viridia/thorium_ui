use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    input::ButtonState,
    input_focus::{FocusKeyboardInput, SetInputFocus},
    prelude::*,
};

/// A "brrier" is a backdrop element, one that covers the entire screen, blocks click events
/// from reaching elements behind it, and can be used to close a dialog or menu.
#[derive(Component)]
pub struct CoreBarrier {
    pub on_close: Option<SystemId>,
}

pub(crate) fn barrier_on_key_input(
    mut trigger: Trigger<FocusKeyboardInput>,
    q_state: Query<&CoreBarrier>,
    mut commands: Commands,
) {
    if let Ok(bstate) = q_state.get(trigger.entity()) {
        let event = &trigger.event().0;
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
    mut trigger: Trigger<Pointer<Down>>,
    q_state: Query<&CoreBarrier>,
    mut world: DeferredWorld,
    mut commands: Commands,
) {
    if let Ok(bstate) = q_state.get(trigger.entity()) {
        let entity_id = trigger.entity();
        world.set_input_focus(entity_id);
        trigger.propagate(false);
        if let Some(on_close) = bstate.on_close {
            commands.run_system(on_close);
        }
    }
}
