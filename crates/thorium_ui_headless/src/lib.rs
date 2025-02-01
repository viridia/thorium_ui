use bevy::{
    app::{App, Plugin, Update},
    input_focus::{tab_navigation, InputDispatchPlugin},
};
mod core_barrier;
mod core_button;
mod core_checkbox;
mod core_radio;
mod core_slider;
mod cursor;
mod disabled;
pub mod handle;
pub mod hover;
mod value_change;

pub use core_barrier::CoreBarrier;
pub use core_button::{CoreButton, CoreButtonPressed};
pub use core_checkbox::CoreCheckbox;
pub use core_radio::CoreRadio;
pub use core_slider::CoreSlider;
pub use disabled::{InteractionDisabled, IsInteractionDisabled};
pub use value_change::ValueChange;

pub struct ThoriumUiHeadlessPlugin;

use core_button::CoreButtonPlugin;
use core_checkbox::CoreCheckboxPlugin;
use core_radio::CoreRadioPlugin;

impl Plugin for ThoriumUiHeadlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputDispatchPlugin)
            .add_plugins(tab_navigation::TabNavigationPlugin)
            .add_plugins((CoreButtonPlugin, CoreCheckboxPlugin, CoreRadioPlugin))
            .add_systems(Update, (hover::update_hover_states, cursor::update_cursor))
            .add_observer(core_barrier::barrier_on_key_input)
            .add_observer(core_barrier::barrier_on_pointer_down)
            .add_observer(core_slider::slider_on_drag_start)
            .add_observer(core_slider::slider_on_drag_end)
            .add_observer(core_slider::slider_on_drag);
    }
}
