use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, system::SystemId, world::DeferredWorld},
    input::{keyboard::KeyboardInput, ButtonState},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::InteractionDisabled;

/// Headless widget implementation for radio buttons. Note that this does not handle the mutual
/// exclusion of radio buttons in the same group; that should be handled by the parent component.
/// (This is relatively easy if the parent is a reactive widget.)
#[derive(Component, Debug)]
#[require(AccessibilityNode(|| AccessibilityNode::from(accesskit::Node::new(Role::CheckBox))))]
#[component(on_add = on_add_radio)]
pub struct CoreRadio {
    pub checked: bool,
    pub on_click: Option<SystemId>,
}

// Hook to set the a11y "checked" state when the radio is added.
fn on_add_radio(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let radio = entt.get::<CoreRadio>().unwrap();
    let checked = radio.checked;
    let mut accessibility = entt.get_mut::<AccessibilityNode>().unwrap();
    accessibility.set_toggled(match checked {
        true => accesskit::Toggled::True,
        false => accesskit::Toggled::False,
    });
}

fn radio_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<(&CoreRadio, Has<InteractionDisabled>)>,
    mut world: DeferredWorld,
) {
    if let Ok((radio, disabled)) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        let is_checked = radio.checked;
        if !disabled
            && event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
            && !is_checked
        {
            if let Some(on_click) = radio.on_click {
                trigger.propagate(false);
                world.commands().run_system(on_click);
            }
        }
    }
}

fn radio_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreRadio, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut world: DeferredWorld,
) {
    if let Ok((radio, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        let is_checked = radio.checked;
        if let Some(on_click) = radio.on_click {
            if !disabled && !is_checked {
                world.commands().run_system(on_click);
            }
        }
    }
}

pub struct CoreRadioPlugin;

impl Plugin for CoreRadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(radio_on_key_input)
            .add_observer(radio_on_pointer_click);
    }
}
