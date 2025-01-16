use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use thorium_ui_core::{StyleEntity, UiTemplate};

fn style_spacer(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.flex_grow = 1.;
    });
}

/// A spacer widget that fills the available space.
#[derive(Clone, Default)]
pub struct Spacer;

impl UiTemplate for Spacer {
    fn build(&self, builder: &mut RelatedSpawnerCommands<Parent>) {
        builder.spawn(Node::default()).style(style_spacer);
    }
}
