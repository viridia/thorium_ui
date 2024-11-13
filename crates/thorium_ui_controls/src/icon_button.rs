use super::{Button, Icon};
use crate::{colors, rounded_corners::RoundedCorners, size::Size};
use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
};
use thorium_ui_core::{
    CreateMemo, IntoSignal, InvokeUiTemplate, Signal, StyleHandle, StyleTuple, UiTemplate,
};
use thorium_ui_headless::handle::HandleOrOwnedPath;

/// A widget which displays a button containing an icon.
#[derive(Default)]
pub struct IconButton {
    /// Asset path for the icon
    pub icon: HandleOrOwnedPath<Image>,

    /// Color variant - default, primary or danger.
    // pub variant: Signal<ButtonVariant>,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: Signal<bool>,

    /// Which corners to render rounded.
    pub corners: RoundedCorners,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_click: Option<SystemId>,

    /// The tab index of the button (default 0).
    pub tab_index: i32,

    /// If true, set focus to this button when it's added to the UI.
    pub autofocus: bool,

    /// If true, render the button in a 'minimal' style with no background and reduced padding.
    pub minimal: bool,
}

impl IconButton {
    /// Construct a new `IconButton`.
    pub fn new(icon: impl Into<HandleOrOwnedPath<Image>>) -> Self {
        Self {
            icon: icon.into(),
            ..default()
        }
    }

    /// Set the button color variant.
    // pub fn variant(mut self, variant: impl IntoSignal<ButtonVariant>) -> Self {
    //     self.variant = variant.into_signal();
    //     self
    // }

    /// Set whether to render the button in a 'minimal' style with no background and reduced padding.
    pub fn minimal(mut self, minimal: bool) -> Self {
        self.minimal = minimal;
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set the button disabled state.
    pub fn disabled(mut self, disabled: impl IntoSignal<bool>) -> Self {
        self.disabled = disabled.into_signal();
        self
    }

    /// Set the additional styles for the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set which corners to render rounded.
    pub fn corners(mut self, corners: RoundedCorners) -> Self {
        self.corners = corners;
        self
    }

    /// Set callback when clicked
    pub fn on_click(mut self, callback: SystemId) -> Self {
        self.on_click = Some(callback);
        self
    }

    /// Set the tab index of the button.
    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }

    /// Set whether to autofocus the button when it's added to the UI.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }
}

impl UiTemplate for IconButton {
    fn build(&self, builder: &mut ChildBuilder) {
        let disabled = self.disabled;
        let size = self.size;
        let icon = self.icon.clone();
        let icon_color = builder.create_memo(
            move |world: DeferredWorld| {
                if disabled.get(&world) {
                    Color::from(colors::DIM).with_alpha(0.2)
                } else {
                    Color::from(colors::DIM)
                }
            },
            Color::from(colors::DIM),
        );

        Button {
            size: self.size,
            disabled,
            style: StyleHandle::new((
                |ent: &mut EntityCommands| {
                    ent.entry::<Node>().and_modify(|mut node| {
                        node.padding = ui::UiRect::axes(ui::Val::Px(4.0), ui::Val::Px(0.0))
                    });
                },
                self.style.clone(),
            )),
            on_click: self.on_click,
            tab_index: self.tab_index,
            autofocus: self.autofocus,
            minimal: self.minimal,
            corners: self.corners,
            ..default()
        }
        .children(move |builder| {
            builder.invoke(Icon::new(icon.clone()).color(icon_color).size(match size {
                Size::Xl => Vec2::splat(20.),
                Size::Lg => Vec2::splat(18.),
                Size::Md => Vec2::splat(16.),
                Size::Sm => Vec2::splat(14.),
                Size::Xs => Vec2::splat(12.),
                Size::Xxs => Vec2::splat(11.),
                Size::Xxxs => Vec2::splat(10.),
            }));
        })
        .build(builder);
    }
}
