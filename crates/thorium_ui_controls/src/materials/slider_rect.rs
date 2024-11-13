use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct SliderRectMaterial {
    #[uniform(0)]
    pub(crate) color_lo: Vec4,
    #[uniform(1)]
    pub(crate) color_hi: Vec4,
    #[uniform(2)]
    pub(crate) value: Vec4,
    #[uniform(3)]
    pub(crate) radius: Vec4, // TopLeft, TopRight, BottomRight, BottomLeft
}

impl UiMaterial for SliderRectMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://thorium_ui_controls/assets/shaders/slider_rect.wgsl".into()
    }
}
