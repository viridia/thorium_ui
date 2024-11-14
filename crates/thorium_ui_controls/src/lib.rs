use bevy::{
    app::{Plugin, PostUpdate},
    asset::embedded_asset,
    ui::UiMaterialPlugin,
};

pub mod animation;
mod button;
mod checkbox;
pub mod colors;
mod disclosure_toggle;
mod gradient_slider;
mod icon;
mod icon_button;
mod image_handle;
mod materials;
pub mod rounded_corners;
pub mod size;
mod slider;
pub mod spacer;
mod spinbox;
mod swatch;
mod swatch_grid;
mod text_styles;
pub mod typography;

use materials::{GradientRectMaterial, SliderRectMaterial, SwatchRectMaterial};
use text_styles::{set_initial_text_style, update_text_styles};
use thorium_ui_headless::ThoriumUiHeadlessPlugin;

pub use button::{Button, ButtonVariant};
pub use checkbox::Checkbox;
pub use disclosure_toggle::DisclosureToggle;
pub use gradient_slider::{ColorGradient, GradientSlider};
pub use icon::Icon;
pub use icon_button::IconButton;
pub use slider::Slider;
pub use spacer::Spacer;
pub use spinbox::SpinBox;
pub use swatch::Swatch;
pub use swatch_grid::SwatchGrid;
pub use text_styles::{
    InheritableFont, InheritableFontColor, InheritableFontSize, UseInheritedTextStyles,
};
pub use thorium_ui_headless::{CoreButtonPressed, ValueChange};

pub struct ThoriumUiControlsPlugin;

impl Plugin for ThoriumUiControlsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Bold.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-BoldItalic.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Medium.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-MediumItalic.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Regular.ttf");
        embedded_asset!(app, "assets/fonts/Fira_Sans/FiraSans-Italic.ttf");

        embedded_asset!(app, "assets/icons/add_box.png");
        embedded_asset!(app, "assets/icons/add.png");
        embedded_asset!(app, "assets/icons/checkmark.png");
        embedded_asset!(app, "assets/icons/chevron_down.png");
        embedded_asset!(app, "assets/icons/chevron_up.png");
        embedded_asset!(app, "assets/icons/chevron_left.png");
        embedded_asset!(app, "assets/icons/chevron_right.png");
        embedded_asset!(app, "assets/icons/close.png");
        embedded_asset!(app, "assets/icons/disc.png");
        embedded_asset!(app, "assets/icons/gradient_thumb.png");
        embedded_asset!(app, "assets/icons/lock.png");
        embedded_asset!(app, "assets/icons/redo.png");
        embedded_asset!(app, "assets/icons/remove.png");
        embedded_asset!(app, "assets/icons/tune.png");
        embedded_asset!(app, "assets/icons/undo.png");
        embedded_asset!(app, "assets/icons/zoom_in.png");
        embedded_asset!(app, "assets/icons/zoom_out.png");
        embedded_asset!(app, "assets/shaders/gradient_rect.wgsl");
        embedded_asset!(app, "assets/shaders/swatch_rect.wgsl");
        embedded_asset!(app, "assets/shaders/slider_rect.wgsl");
        app.add_plugins((
            UiMaterialPlugin::<GradientRectMaterial>::default(),
            UiMaterialPlugin::<SliderRectMaterial>::default(),
            UiMaterialPlugin::<SwatchRectMaterial>::default(),
            // animation::BistableTransitionPlugin,
            animation::AnimatedTransitionPlugin,
        ));

        app.add_plugins(ThoriumUiHeadlessPlugin);
        app.world_mut().add_observer(set_initial_text_style);
        app.add_systems(PostUpdate, update_text_styles);
    }
}
