use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    input::ButtonState,
    input_focus::{FocusKeyboardInput, SetInputFocus},
    prelude::*,
};
use thorium_ui_core::Signal;

use crate::InteractionDisabled;

#[derive(Component)]
pub struct CoreToggle {
    pub checked: Signal<bool>,
    pub on_change: Option<SystemId<In<bool>>>,
}

pub(crate) fn toggle_on_key_input(
    mut trigger: Trigger<FocusKeyboardInput>,
    q_state: Query<(&CoreToggle, Has<InteractionDisabled>)>,
    mut world: DeferredWorld,
) {
    if let Ok((tstate, disabled)) = q_state.get(trigger.entity()) {
        let event = &trigger.event().0;
        if !disabled
            && event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
        {
            let is_checked = tstate.checked.get(&world);
            if let Some(on_change) = tstate.on_change {
                trigger.propagate(false);
                world
                    .commands()
                    .run_system_with_input(on_change, !is_checked);
            }
        }
    }
}

pub(crate) fn toggle_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreToggle, Has<InteractionDisabled>)>,
    mut world: DeferredWorld,
) {
    if let Ok((tstate, disabled)) = q_state.get(trigger.entity()) {
        let checkbox_id = trigger.entity();
        world.set_input_focus(checkbox_id);
        trigger.propagate(false);
        if let Some(on_change) = tstate.on_change {
            if !disabled {
                let is_checked = tstate.checked.get(&world);
                world
                    .commands()
                    .run_system_with_input(on_change, !is_checked);
            }
        }
    }
}
