use bevy::app::{App, Plugin, Update};
pub mod hover;

pub struct ThoriumUiHeadlessPlugin;

impl Plugin for ThoriumUiHeadlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover::update_hover_states);
    }
}
