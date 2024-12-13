use bevy::{
    ecs::system::SystemId,
    input_focus::{FocusKeyboardInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::InteractionDisabled;

/// Headless button widget. The `on_click` field is a system that will be run when the button
/// is clicked, or when the Enter or Space key is pressed while the button is focused.
#[derive(Component, Debug)]
#[require(CoreButtonPressed)]
pub struct CoreButton {
    pub on_click: Option<SystemId>,
}

#[derive(Component, Default, Debug)]
pub struct CoreButtonPressed(pub bool);

pub(crate) fn button_on_key_event(
    mut trigger: Trigger<FocusKeyboardInput>,
    q_state: Query<(&CoreButton, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((bstate, disabled)) = q_state.get(trigger.target()) {
        if !disabled {
            let event = &trigger.event().input;
            if !event.repeat
                && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
            {
                if let Some(on_click) = bstate.on_click {
                    trigger.propagate(false);
                    commands.run_system(on_click);
                }
            }
        }
    }
}

pub(crate) fn button_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut q_state: Query<(
        &CoreButton,
        &mut CoreButtonPressed,
        Has<InteractionDisabled>,
    )>,
    mut commands: Commands,
) {
    if let Ok((bstate, pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if pressed.0 && !disabled {
            // println!("Click: {}", pressed.0);
            if let Some(on_click) = bstate.on_click {
                commands.run_system(on_click);
            }
        }
    }
}

pub(crate) fn button_on_pointer_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    mut q_state: Query<(&mut CoreButtonPressed, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = true;
            focus.0 = Some(trigger.target());
            focus_visible.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_up(
    mut trigger: Trigger<Pointer<Released>>,
    mut q_state: Query<(&mut CoreButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut q_state: Query<(&mut CoreButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_cancel(
    mut trigger: Trigger<Pointer<Cancel>>,
    mut q_state: Query<(&mut CoreButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}
