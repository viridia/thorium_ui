use bevy::prelude::*;
use thorium_ui_core::{Template, TemplateContext};

/// A spacer widget that fills the available space.
#[derive(Clone, Default)]
pub struct Spacer;

impl Template for Spacer {
    fn build(&self, builder: &mut TemplateContext) {
        builder.spawn(Node {
            flex_grow: 1.,
            ..Default::default()
        });
    }
}
