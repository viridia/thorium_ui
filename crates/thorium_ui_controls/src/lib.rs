use bevy::{
    app::{Plugin, PostUpdate},
    asset::embedded_asset,
};

pub mod colors;
pub mod rounded_corners;
pub mod size;
mod text_styles;
pub mod typography;

mod button;

pub use button::{Button, ButtonVariant};
use text_styles::{set_initial_text_style, update_text_styles};
pub use text_styles::{
    InheritableFont, InheritableFontColor, InheritableFontSize, UseInheritedTextStyles,
};
use thorium_ui_headless::ThoriumUiHeadlessPlugin;
pub use thorium_ui_headless::{CoreButton, CoreButtonPressed};

pub struct ThoriumUiControlsPlugin;

impl Plugin for ThoriumUiControlsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Bold.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-BoldItalic.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Medium.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-MediumItalic.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Regular.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Italic.ttf");

        app
        .add_plugins(ThoriumUiHeadlessPlugin)
            // .add_observer(toggle_state::toggle_on_key_input)
            // .add_observer(toggle_state::toggle_on_pointer_click)
        // .add_observer(barrier::barrier_on_key_input)
        // .add_observer(barrier::barrier_on_pointer_down)
        // .add_observer(core_slider::slider_on_drag_start)
        // .add_observer(core_slider::slider_on_drag_end)
        // .add_observer(core_slider::slider_on_drag);
        ;
        app.world_mut().add_observer(set_initial_text_style);
        app.add_systems(PostUpdate, update_text_styles);
    }
}
