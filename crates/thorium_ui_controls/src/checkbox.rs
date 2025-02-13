use std::sync::Arc;

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
    computations, Calc, Cond, InsertWhen, IntoSignal, Signal, SpawnArc, SpawnableListGen,
    StyleHandle, StyleTuple, Styles, Template, TemplateContext,
};
use thorium_ui_headless::{
    hover::{Hovering, IsHovering},
    CoreCheckbox, InteractionDisabled,
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
#[derive(Default)]
pub struct Checkbox {
    /// Whether the checkbox is checked.
    pub checked: Signal<bool>,

    /// Whether the checkbox is disabled.
    pub disabled: Signal<bool>,

    /// The content to display inside the button.
    pub label: Option<Arc<dyn SpawnableListGen + Send + Sync>>,

    /// ARIA label for this checkbox.
    pub aria_label: Option<String>,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_change: Option<SystemId<In<bool>>>,

    /// The tab index of the checkbox (default 0).
    pub tab_index: i32,
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

    /// Set the label of the checkbox from a [`SpawnableList`].
    pub fn label<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.label = Some(Arc::new(elts));
        self
    }

    /// Set the label of the checkbox from a string.
    pub fn labeled(mut self, label: impl Into<String>) -> Self {
        let s: String = label.into();
        self.label = Some(Arc::new(move || {
            Spawn((Text::new(s.clone()), UseInheritedTextStyles))
        }));
        self
    }

    /// Set the ARIA label of the checkbox from a string.
    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
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

impl Template for Checkbox {
    /// Construct a checkbox widget.
    fn build(&self, builder: &mut TemplateContext) {
        let checked = self.checked;
        let disabled = self.disabled;
        let mut checkbox = builder.spawn((
            Node { ..default() },
            Hovering::default(),
            Name::new("Checkbox"),
            // AccessibilityNode::from(accesskit::Node::new(Role::CheckBox)).set_label(
            //     self.aria_label
            //         .clone()
            //         .unwrap_or_else(|| "Checkbox".to_string()),
            // ),
            Styles((style_checkbox, self.style.clone())),
            TabIndex(self.tab_index),
            CoreCheckbox {
                checked: false,
                on_change: self.on_change,
            },
            computations![
                Calc::new(
                    move |world: DeferredWorld| checked.get(&world),
                    |checked, ent| {
                        let checkbox = ent.get::<CoreCheckbox>().unwrap();
                        ent.insert(CoreCheckbox {
                            checked,
                            ..*checkbox
                        });
                    },
                ),
                InsertWhen::new(
                    move |world: DeferredWorld| disabled.get(&world),
                    || InteractionDisabled,
                ),
            ],
        ));
        let checkbox_id = checkbox.id();

        // Set ARIA label.
        if let Some(aria_label) = &self.aria_label {
            if let Some(mut access_node) = checkbox.get_mut::<AccessibilityNode>() {
                access_node.set_label(aria_label.clone());
            }
        }

        checkbox.with_related::<ChildOf>(|builder| {
            builder.spawn((
                Node { ..default() },
                Name::new("Checkbox::Border"),
                Styles(style_checkbox_border),
                computations![
                    Calc::new(
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
                    Calc::new(
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
                    ),
                ],
                children![Cond::new(
                    move |world: DeferredWorld| checked.get(&world),
                    move || Spawn((
                        ImageNode {
                            color: Srgba::WHITE.into(),
                            ..default()
                        },
                        UiImageHandle(
                            "embedded://thorium_ui_controls/assets/icons/checkmark.png".into(),
                        ),
                        Styles(style_checkbox_inner),
                    )),
                    || (),
                )],
            ));

            builder.spawn((
                Node::default(),
                Styles((typography::text_default, style_checkbox_label)),
                computations![Calc::new(
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
                ),],
                Children::spawn(SpawnArc(self.label.clone())),
            ));
        });
    }
}
