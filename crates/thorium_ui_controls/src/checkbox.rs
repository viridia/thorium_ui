use std::sync::Arc;

use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    color::Luminance,
    ecs::{system::SystemId, world::DeferredWorld},
    input_focus::{tab_navigation::TabIndex, IsFocused},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::{
    BuildEffects, CreateCond, InsertWhen, IntoSignal, Signal, StyleDyn, StyleEntity, StyleHandle,
    StyleTuple, UiTemplate,
};
use thorium_ui_headless::{
    hover::{Hovering, IsHovering},
    CoreToggle, InteractionDisabled,
};

use crate::{
    colors, image_handle::UiImageHandle, typography, InheritableFontColor, InheritableFontSize,
    UseInheritedTextStyles,
};

fn style_checkbox(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.justify_content = ui::JustifyContent::FlexStart;
        node.align_items = ui::AlignItems::Center;
        node.align_content = ui::AlignContent::Center;
        node.column_gap = ui::Val::Px(8.0);
    });
    ec.insert(CursorIcon::System(SystemCursorIcon::Pointer));
}

fn style_checkbox_border(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.width = ui::Val::Px(16.0);
        node.height = ui::Val::Px(16.0);
    });
    ec.insert(BorderRadius::all(ui::Val::Px(3.0)));
}

fn style_checkbox_inner(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.position_type = ui::PositionType::Absolute;
        node.left = ui::Val::Px(2.0);
        node.top = ui::Val::Px(2.0);
        node.width = ui::Val::Px(12.0);
        node.height = ui::Val::Px(12.0);
    });
}

fn style_checkbox_label(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.justify_content = ui::JustifyContent::FlexStart;
        node.align_items = ui::AlignItems::Center;
        node.column_gap = ui::Val::Px(8.0);
    });
    ec.insert(InheritableFontColor(colors::FOREGROUND.into()));
    ec.insert(InheritableFontSize(14.0));
}

/// A checkbox widget.
pub struct Checkbox {
    /// Whether the checkbox is checked.
    pub checked: Signal<bool>,

    /// Whether the checkbox is disabled.
    pub disabled: Signal<bool>,

    /// The content to display inside the button.
    pub label: Arc<dyn Fn(&mut ChildBuilder)>,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_change: Option<SystemId<In<bool>>>,

    /// The tab index of the checkbox (default 0).
    pub tab_index: i32,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            checked: Default::default(),
            disabled: Default::default(),
            label: Arc::new(|_builder| {}),
            style: Default::default(),
            on_change: Default::default(),
            tab_index: Default::default(),
        }
    }
}

impl Checkbox {
    /// Create a new checkbox.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the checked state of the checkbox.
    pub fn checked(mut self, checked: impl IntoSignal<bool>) -> Self {
        self.checked = checked.into_signal();
        self
    }

    /// Set the disabled state of the checkbox.
    pub fn disabled(mut self, disabled: impl IntoSignal<bool>) -> Self {
        self.disabled = disabled.into_signal();
        self
    }

    /// Set the label of the checkbox.
    pub fn labeled(mut self, label: impl Into<String>) -> Self {
        let s: String = label.into();
        self.label = Arc::new(move |builder| {
            // TODO: Figure out how to avoid the double-copy here.
            builder.spawn((Text::new(s.clone()), UseInheritedTextStyles));
        });
        self
    }

    /// Set the style of the checkbox.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the on_change callback of the checkbox.
    pub fn on_change(mut self, on_change: SystemId<In<bool>>) -> Self {
        self.on_change = Some(on_change);
        self
    }

    /// Set the tab index of the checkbox.
    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

impl UiTemplate for Checkbox {
    /// Construct a checkbox widget.
    fn build(&self, builder: &mut ChildBuilder) {
        let mut checkbox =
            builder.spawn((Node::default(), Hovering::default(), Name::new("Checkbox")));
        let checkbox_id = checkbox.id();

        let checked = self.checked;
        let disabled = self.disabled;

        checkbox
            .style((style_checkbox, self.style.clone()))
            .insert((
                TabIndex(self.tab_index),
                CoreToggle {
                    checked,
                    on_change: self.on_change,
                },
                AccessibilityNode::from(accesskit::Node::new(Role::CheckBox)),
            ))
            .effects(InsertWhen::new(
                move |world: DeferredWorld| disabled.get(&world),
                || InteractionDisabled,
            ))
            // .insert_if(AutoFocus, || self.autofocus)
            .with_children(|builder| {
                builder
                    .spawn((Node::default(), Name::new("Checkbox::Border")))
                    .style(style_checkbox_border)
                    .effects((
                        StyleDyn::new(
                            move |world: DeferredWorld| match (
                                checked.get(&world),
                                disabled.get(&world),
                                world.is_hovering(checkbox_id),
                            ) {
                                (true, true, _) => colors::ACCENT.with_alpha(0.2),
                                (true, false, true) => colors::ACCENT.darker(0.15),
                                (true, _, _) => colors::ACCENT.darker(0.2),
                                (false, true, _) => colors::U1.with_alpha(0.7),
                                (false, false, true) => colors::U1.lighter(0.002),
                                (false, false, false) => colors::U1,
                            },
                            |color, ec| {
                                ec.insert(BackgroundColor(color.into()));
                            },
                        ),
                        StyleDyn::new(
                            move |world: DeferredWorld| world.is_focus_visible(checkbox_id),
                            |is_focused, ec| {
                                if is_focused {
                                    ec.insert(Outline {
                                        color: colors::FOCUS.into(),
                                        width: ui::Val::Px(2.0),
                                        offset: ui::Val::Px(2.0),
                                    });
                                } else {
                                    ec.remove::<Outline>();
                                };
                            },
                        )
                    ))
                    .with_children(|builder| {
                        builder.cond(
                            move |world: DeferredWorld| checked.get(&world),
                            move |builder| {
                                builder
                                    .spawn((ImageNode {
                                        color: Srgba::WHITE.into(),
                                        ..default()
                                    },
                                    UiImageHandle("embedded://thorium_ui_controls/assets/icons/checkmark.png".into())))
                                    .style(style_checkbox_inner);
                            },
                            |_| {},
                        );
                    });

                builder
                    .spawn(Node::default())
                    .style((typography::text_default, style_checkbox_label))
                    .effects(
                        StyleDyn::new(
                            move |world: DeferredWorld| disabled.get(&world),
                            |disabled, ec| {
                                ec.entry::<InheritableFontColor>()
                                    .and_modify(move |mut color| {
                                        if disabled {
                                            color.0 = colors::FOREGROUND.with_alpha(0.2).into();
                                        } else {
                                            color.0 = colors::FOREGROUND.into();
                                        }
                                    });
                            },

                        )
                    )
                    .with_children(|builder| {
                        (self.label.as_ref())(builder);
                    });
            });
    }
}
