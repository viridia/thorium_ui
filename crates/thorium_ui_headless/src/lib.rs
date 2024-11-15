use bevy::app::{App, Plugin, Update};
mod core_barrier;
mod core_button;
mod core_slider;
mod core_toggle;
mod cursor;
mod disabled;
pub mod focus;
pub mod handle;
pub mod hover;
pub mod tab_navigation;
mod value_change;

pub use core_barrier::CoreBarrier;
pub use core_button::{CoreButton, CoreButtonPressed};
pub use core_slider::CoreSlider;
pub use core_toggle::CoreToggle;
pub use disabled::{InteractionDisabled, IsInteractionDisabled};
use focus::InputDispatchPlugin;
pub use focus::{SetFocusCommand, SetKeyboardFocus};
use tab_navigation::KeyboardFocusVisible;
pub use value_change::ValueChange;

pub struct ThoriumUiHeadlessPlugin;

impl Plugin for ThoriumUiHeadlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputDispatchPlugin)
            .add_systems(Update, (hover::update_hover_states, cursor::update_cursor))
            .add_observer(core_button::button_on_key_event)
            .add_observer(core_button::button_on_pointer_down)
            .add_observer(core_button::button_on_pointer_up)
            .add_observer(core_button::button_on_pointer_click)
            .add_observer(core_button::button_on_pointer_drag_end)
            .add_observer(core_button::button_on_pointer_cancel)
            .add_observer(core_toggle::toggle_on_key_input)
            .add_observer(core_toggle::toggle_on_pointer_click)
            .add_observer(core_barrier::barrier_on_key_input)
            .add_observer(core_barrier::barrier_on_pointer_down)
            .add_observer(core_slider::slider_on_drag_start)
            .add_observer(core_slider::slider_on_drag_end)
            .add_observer(core_slider::slider_on_drag)
            .insert_resource(KeyboardFocusVisible(false));
    }
}
