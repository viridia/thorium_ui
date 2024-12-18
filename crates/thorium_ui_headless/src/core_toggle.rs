use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    input::ButtonState,
    input_focus::{FocusKeyboardInput, InputFocus, InputFocusVisible},
    prelude::*,
};
use thorium_ui_core::Signal;

use crate::InteractionDisabled;

#[derive(Component, Debug)]
pub struct CoreToggle {
    pub checked: Signal<bool>,
    pub on_change: Option<SystemId<In<bool>>>,
}

pub(crate) fn toggle_on_key_input(
    mut trigger: Trigger<FocusKeyboardInput>,
    q_state: Query<(&CoreToggle, Has<InteractionDisabled>)>,
    mut world: DeferredWorld,
) {
    if let Ok((tstate, disabled)) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if !disabled
            && event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
        {
            let is_checked = tstate.checked.get(&world);
            if let Some(on_change) = tstate.on_change {
                trigger.propagate(false);
                world.commands().run_system_with(on_change, !is_checked);
            }
        }
    }
}

pub(crate) fn toggle_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreToggle, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut world: DeferredWorld,
) {
    if let Ok((tstate, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if let Some(on_change) = tstate.on_change {
            if !disabled {
                let is_checked = tstate.checked.get(&world);
                world.commands().run_system_with(on_change, !is_checked);
            }
        }
    }
}
