use bevy::app::{App, Plugin, Update};
mod disabled;
pub mod focus;
pub mod hover;
pub mod tab_navigation;

pub use disabled::{InteractionDisabled, IsDisabled};
use focus::InputDispatchPlugin;
use tab_navigation::KeyboardFocusVisible;

pub struct ThoriumUiHeadlessPlugin;

impl Plugin for ThoriumUiHeadlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputDispatchPlugin)
            .add_systems(Update, hover::update_hover_states)
            .insert_resource(KeyboardFocusVisible(false));
    }
}
