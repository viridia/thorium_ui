use std::sync::Arc;

use crate::{
    colors, rounded_corners::RoundedCorners, size::Size, text_styles::UseInheritedTextStyles,
    typography, InheritableFont, InheritableFontColor, InheritableFontSize,
};
use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    color::Luminance,
    ecs::{system::SystemId, world::DeferredWorld},
    input_focus::{tab_navigation::TabIndex, AutoFocus, IsFocused},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::{
    owned, DynChildren, IndirectSpawnableList, InsertWhen2, IntoSignal, InvokeIndirect, Signal,
    StyleDyn, StyleHandle, StyleTuple, Styles, Template, TemplateContext,
};
use thorium_ui_headless::{
    hover::{Hovering, IsHovering},
    CoreButton, CoreButtonPressed, InteractionDisabled, IsInteractionDisabled,
};

/// The variant determines the button's color scheme
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    /// The default button apperance.
    #[default]
    Default,

    /// A more prominent, "call to action", appearance.
    Primary,

    /// An appearance indicating a potentially dangerous action.
    Danger,

    /// A button that is in a "toggled" state.
    Selected,
}

/// Button widget
pub struct Button {
    /// Color variant - default, primary or danger.
    pub variant: Signal<ButtonVariant>,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: Signal<bool>,

    /// The content to display inside the button.
    pub contents: Option<Arc<dyn IndirectSpawnableList + Send + Sync>>,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_click: Option<SystemId>,

    /// The tab index of the button (default 0).
    pub tab_index: i32,

    /// Which corners to render rounded.
    pub corners: RoundedCorners,

    /// If true, set focus to this button when it's added to the UI.
    pub autofocus: bool,

    /// If true, render the button in a 'minimal' style with no background and reduced padding.
    pub minimal: bool,
}

impl Button {
    /// Construct a new `Button`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the button color variant.
    pub fn variant(mut self, variant: impl IntoSignal<ButtonVariant>) -> Self {
        self.variant = variant.into_signal();
        self
    }

    /// Method which switches between `default` and `selected` style variants based on a boolean.
    /// Often used for toggle buttons or toolbar items.
    pub fn selected(mut self, selected: bool) -> Self {
        self.variant = if selected {
            ButtonVariant::Selected
        } else {
            ButtonVariant::Default
        }
        .into_signal();
        self
    }

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

    /// Set the child views for this element.
    pub fn contents<L: IndirectSpawnableList + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }

    /// Set a child which is a text label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        let s: String = label.into();
        self.contents = Some(Arc::new(move || {
            Spawn((Text::new(s.clone()), UseInheritedTextStyles))
        }));
        self
    }

    /// Set the additional styles for the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
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

    /// Set which corners to render rounded.
    pub fn corners(mut self, corners: RoundedCorners) -> Self {
        self.corners = corners;
        self
    }

    /// Set whether to autofocus the button when it's added to the UI.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            variant: Signal::default(),
            size: Size::default(),
            disabled: default(),
            contents: None,
            style: StyleHandle::none(),
            on_click: None,
            tab_index: 0,
            corners: RoundedCorners::default(),
            autofocus: false,
            minimal: false,
        }
    }
}

impl Template for Button {
    fn build(&self, builder: &mut TemplateContext) {
        let variant = self.variant;
        let corners = self.corners;
        let minimal = self.minimal;
        let disabled = self.disabled;
        let size = self.size;
        let on_click = self.on_click;

        let mut button = builder.spawn((
            Node {
                display: ui::Display::Flex,
                flex_direction: ui::FlexDirection::Row,
                justify_content: ui::JustifyContent::Center,
                align_items: ui::AlignItems::Center,
                align_content: ui::AlignContent::Center,
                padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
                border: ui::UiRect::all(ui::Val::Px(0.0)),
                ..default()
            },
            Name::new("Button"),
            // Marker to indicate we want to be notified when the button, or any child, is hovered.
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            // Child elements should inherit font styles from the parent, unless overridden.
            InheritableFont::from_path(typography::DEFAULT_FONT),
            InheritableFontColor(colors::FOREGROUND.into()),
            InheritableFontSize(size.font_size()),
            TabIndex(self.tab_index),
            // Button pressed state.
            CoreButtonPressed(false),
            // Button behaviors and observers.
            CoreButton { on_click },
            AccessibilityNode::from(accesskit::Node::new(Role::Button)),
            owned![InsertWhen2::new(
                move |world: DeferredWorld| disabled.get(&world),
                || InteractionDisabled,
            )],
            Styles((
                // Calculate button size based on `size` enum.
                move |ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(move |mut node| {
                        node.min_height = ui::Val::Px(size.height());
                        node.min_width = ui::Val::Px(size.height().floor());
                        if minimal {
                            node.padding = ui::UiRect::all(ui::Val::Px(0.0));
                        } else {
                            node.padding = ui::UiRect::axes(
                                ui::Val::Px(size.font_size() * 0.75),
                                ui::Val::Px(0.0),
                            );
                        }
                    });
                },
                // Style overrides passed in by the user.
                self.style.clone(),
            )),
        ));
        let button_id = button.id();

        if self.autofocus {
            button.insert(AutoFocus);
        }

        button.insert(DynChildren::spawn((
            Spawn((
                Node {
                    display: ui::Display::Grid,
                    position_type: ui::PositionType::Absolute,
                    left: ui::Val::Px(0.0),
                    right: ui::Val::Px(0.0),
                    top: ui::Val::Px(0.0),
                    bottom: ui::Val::Px(0.0),
                    ..default()
                },
                Name::new("Button::Background"),
                corners.to_border_radius(self.size.border_radius()),
                StyleDyn::new(
                    move |world: DeferredWorld| {
                        if minimal {
                            colors::TRANSPARENT
                        } else {
                            let entity = world.entity(button_id);
                            let pressed = entity
                                .get::<CoreButtonPressed>()
                                .map(|pressed| pressed.0)
                                .unwrap_or_default();
                            button_bg_color(
                                variant.get(&world),
                                world.is_interaction_disabled(button_id),
                                pressed,
                                world.is_hovering(button_id),
                            )
                        }
                    },
                    |color, ent| {
                        ent.insert(BackgroundColor(color.into()));
                    },
                ),
                StyleDyn::new(
                    move |world: DeferredWorld| world.is_focus_visible(button_id),
                    |is_focused, ent| {
                        if is_focused {
                            ent.insert(Outline {
                                color: colors::FOCUS.into(),
                                width: ui::Val::Px(2.0),
                                offset: ui::Val::Px(2.0),
                            });
                        } else {
                            ent.remove::<Outline>();
                        };
                    },
                ),
            )),
            InvokeIndirect(self.contents.clone()),
        )));
    }
}

pub(crate) fn button_bg_color(
    variant: ButtonVariant,
    is_disabled: bool,
    is_pressed: bool,
    is_hovering: bool,
) -> Srgba {
    let base_color = match variant {
        ButtonVariant::Default => colors::U3,
        ButtonVariant::Primary => colors::PRIMARY,
        ButtonVariant::Danger => colors::DESTRUCTIVE,
        ButtonVariant::Selected => colors::U4,
    };
    // println!("Disabled: {}", is_disabled);
    match (is_disabled, is_pressed, is_hovering) {
        (true, _, _) => base_color.with_alpha(0.2),
        (_, true, true) => base_color.lighter(0.07),
        (_, false, true) => base_color.lighter(0.03),
        _ => base_color,
    }
}
