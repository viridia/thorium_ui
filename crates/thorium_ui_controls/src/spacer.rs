use bevy::prelude::*;
use thorium_ui_core::UiTemplate;

/// A spacer widget that fills the available space.
#[derive(Clone, Default)]
pub struct Spacer;

impl UiTemplate for Spacer {
    fn build(&self, builder: &mut ChildSpawnerCommands) {
        builder.spawn(Node {
            flex_grow: 1.,
            ..Default::default()
        });
    }
}
