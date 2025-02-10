use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, system::SystemId, world::DeferredWorld},
    input::{keyboard::KeyboardInput, ButtonState},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::InteractionDisabled;

/// Headless widget implementation for checkboxes.
#[derive(Component, Debug)]
#[require(AccessibilityNode(|| AccessibilityNode::from(accesskit::Node::new(Role::CheckBox))))]
#[component(on_add = on_add_checkbox, on_replace = on_add_checkbox)]
pub struct CoreCheckbox {
    pub checked: bool,
    pub on_change: Option<SystemId<In<bool>>>,
}

// Hook to set the a11y "checked" state when the checkbox is added.
fn on_add_checkbox(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let checkbox = entt.get::<CoreCheckbox>().unwrap();
    let checked = checkbox.checked;
    let mut accessibility = entt.get_mut::<AccessibilityNode>().unwrap();
    accessibility.set_toggled(match checked {
        true => accesskit::Toggled::True,
        false => accesskit::Toggled::False,
    });
}

fn checkbox_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<(&CoreCheckbox, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((checkbox, disabled)) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if !disabled
            && event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
        {
            let is_checked = checkbox.checked;
            if let Some(on_change) = checkbox.on_change {
                trigger.propagate(false);
                commands.run_system_with(on_change, !is_checked);
            }
        }
    }
}

fn checkbox_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreCheckbox, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((checkbox, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if let Some(on_change) = checkbox.on_change {
            if !disabled {
                let is_checked = checkbox.checked;
                commands.run_system_with(on_change, !is_checked);
            }
        }
    }
}

pub struct CoreCheckboxPlugin;

impl Plugin for CoreCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(checkbox_on_key_input)
            .add_observer(checkbox_on_pointer_click);
    }
}
