use bevy::{ecs::world::DeferredWorld, prelude::*, ui};
use thorium_ui_core::{
    IntoSignal, Signal, StyleDyn, StyleHandle, StyleTuple, Styles, Template, TemplateContext,
    UiTemplate,
};
use thorium_ui_headless::handle::HandleOrOwnedPath;

use crate::{colors, image_handle::UiImageHandle};

/// Control that displays an icon.
#[derive(Clone)]
pub struct Icon {
    /// Asset path for the icon
    pub icon: HandleOrOwnedPath<Image>,

    /// Size of the icon in pixels.
    pub size: Vec2,

    /// Color of the icon.
    pub color: Signal<Color>,

    /// Additional styles to apply to the icon
    pub style: StyleHandle,
}

impl Icon {
    /// Create a new `Icon` from a `&str` or `Handle<Image>`.
    pub fn new(icon: impl Into<HandleOrOwnedPath<Image>>) -> Self {
        Self {
            icon: icon.into(),
            ..default()
        }
    }

    /// Set the size of the icon.
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    /// Set the color of the icon.
    pub fn color(mut self, color: impl IntoSignal<Color>) -> Self {
        self.color = color.into_signal();
        self
    }

    /// Set the style of the icon.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            icon: HandleOrOwnedPath::default(),
            size: Vec2::splat(12.0),
            color: Signal::Constant(colors::FOREGROUND.into()),
            style: StyleHandle::default(),
        }
    }
}

impl UiTemplate for Icon {
    fn build(&self, builder: &mut ChildSpawnerCommands) {
        let icon = self.icon.clone();
        let size = self.size;
        let color = self.color;

        builder.spawn((
            ImageNode { ..default() },
            UiImageHandle(icon),
            Styles((
                move |ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(move |mut node| {
                        node.width = ui::Val::Px(size.x);
                        node.height = ui::Val::Px(size.y);
                    });
                },
                self.style.clone(),
            )),
            StyleDyn::new(
                move |world: DeferredWorld| color.get(&world),
                |color, ent| {
                    ent.entry::<ImageNode>().and_modify(move |mut img| {
                        img.color = color;
                    });
                },
            ),
        ));
    }
}

impl Template for Icon {
    fn build(&self, builder: &mut TemplateContext) {
        let icon = self.icon.clone();
        let size = self.size;
        let color = self.color;

        builder.spawn((
            ImageNode { ..default() },
            UiImageHandle(icon),
            Styles((
                move |ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(move |mut node| {
                        node.width = ui::Val::Px(size.x);
                        node.height = ui::Val::Px(size.y);
                    });
                },
                self.style.clone(),
            )),
            StyleDyn::new(
                move |world: DeferredWorld| color.get(&world),
                |color, ent| {
                    ent.entry::<ImageNode>().and_modify(move |mut img| {
                        img.color = color;
                    });
                },
            ),
        ));
    }
}
