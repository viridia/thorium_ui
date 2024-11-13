use bevy::app::{App, Plugin, Update};
mod core_button;
mod core_slider;
mod disabled;
pub mod focus;
pub mod handle;
pub mod hover;
pub mod tab_navigation;

pub use core_button::{CoreButton, CoreButtonPressed};
pub use core_slider::CoreSlider;
pub use disabled::{InteractionDisabled, IsInteractionDisabled};
use focus::InputDispatchPlugin;
use tab_navigation::KeyboardFocusVisible;

pub struct ThoriumUiHeadlessPlugin;

impl Plugin for ThoriumUiHeadlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputDispatchPlugin)
            .add_systems(Update, hover::update_hover_states)
            .add_observer(core_button::button_on_key_event)
            .add_observer(core_button::button_on_pointer_down)
            .add_observer(core_button::button_on_pointer_up)
            .add_observer(core_button::button_on_pointer_click)
            .add_observer(core_button::button_on_pointer_drag_end)
            .add_observer(core_button::button_on_pointer_cancel)
            // .add_observer(toggle_state::toggle_on_key_input)
            // .add_observer(toggle_state::toggle_on_pointer_click)
            // .add_observer(barrier::barrier_on_key_input)
            // .add_observer(barrier::barrier_on_pointer_down)
            .add_observer(core_slider::slider_on_drag_start)
            .add_observer(core_slider::slider_on_drag_end)
            .add_observer(core_slider::slider_on_drag)
            .insert_resource(KeyboardFocusVisible(false));
    }
}
