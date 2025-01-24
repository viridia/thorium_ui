use crate::{
    animation::{AnimatedRotation, AnimatedTransition},
    colors,
    size::Size,
    Icon,
};
use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    input_focus::{tab_navigation::TabIndex, IsFocused},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::{
    Attach, CreateMemo, IntoSignal, InvokeUiTemplate, MutateDyn, Signal, StyleDyn, StyleEntity,
    StyleHandle, StyleTuple, UiTemplate,
};
use thorium_ui_headless::{hover::IsHovering, CoreCheckbox};

fn style_toggle(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.justify_content = ui::JustifyContent::Center;
        node.align_items = ui::AlignItems::Center;
        node.align_content = ui::AlignContent::Center;
    });
    ec.insert(CursorIcon::System(SystemCursorIcon::Pointer));
}

/// A widget which displays small toggleable chevron that can be used to control whether
/// a panel is visible or hidden.
#[derive(Default)]
pub struct DisclosureToggle {
    /// Whether the toggle is in an expanded state.
    pub expanded: Signal<bool>,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: Signal<bool>,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when the state is toggled
    pub on_change: Option<SystemId<In<bool>>>,

    /// The tab index of the button (default 0).
    pub tab_index: i32,

    /// If true, set focus to this button when it's added to the UI.
    pub autofocus: bool,
}

impl DisclosureToggle {
    /// Construct a new `DisclosureToggle`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the expanded state of the button.
    pub fn expanded(mut self, expanded: impl IntoSignal<bool>) -> Self {
        self.expanded = expanded.into_signal();
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

    /// Set callback when clicked
    pub fn on_change(mut self, callback: SystemId<In<bool>>) -> Self {
        self.on_change = Some(callback);
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

impl UiTemplate for DisclosureToggle {
    fn build(&self, builder: &mut ChildSpawnerCommands) {
        let disabled = self.disabled;
        let checked = self.expanded;
        let mut toggle = builder.spawn((Node::default(), Name::new("DisclosureToggle")));
        let toggle_id = toggle.id();

        toggle
            .style((style_toggle, self.style.clone()))
            .insert((
                CoreCheckbox {
                    on_change: self.on_change,
                    checked: false,
                },
                TabIndex(self.tab_index),
            ))
            .attach(MutateDyn::new(
                move |world: DeferredWorld| checked.get(&world),
                |checked, ent| {
                    let angle = if checked {
                        std::f32::consts::PI * 0.5
                    } else {
                        0.
                    };
                    let target = Quat::from_rotation_z(angle);
                    AnimatedTransition::<AnimatedRotation>::start(ent, target, None, 0.3);
                    let mut checkbox = ent.get_mut::<CoreCheckbox>().unwrap();
                    checkbox.checked = checked;
                },
            ))
            .attach(StyleDyn::new(
                move |world: DeferredWorld| world.is_focus_visible(toggle_id),
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
            ))
            .with_children(|builder| {
                let icon_color = builder.create_memo(
                    move |world: DeferredWorld| {
                        let is_disabled = disabled.get(&world);
                        let is_hover = world.is_hovering(toggle_id);
                        match (is_disabled, is_hover) {
                            (true, _) => Color::from(colors::DIM).with_alpha(0.2),
                            (false, true) => Color::from(colors::FOREGROUND),
                            (false, false) => Color::from(colors::DIM),
                        }
                    },
                    Color::from(colors::FOREGROUND),
                );

                builder.invoke(
                    Icon::new("embedded://thorium_ui_controls/assets/icons/chevron_right.png")
                        .color(icon_color)
                        .size(match self.size {
                            Size::Xl => Vec2::splat(24.),
                            Size::Lg => Vec2::splat(20.),
                            Size::Md => Vec2::splat(18.),
                            Size::Sm => Vec2::splat(16.),
                            Size::Xs => Vec2::splat(13.),
                            Size::Xxs => Vec2::splat(12.),
                            Size::Xxxs => Vec2::splat(11.),
                        })
                        .style(|ec: &mut EntityCommands| {
                            ec.entry::<Node>().and_modify(|mut node| {
                                node.margin.right = ui::Val::Px(2.);
                            });
                        }),
                );
            });
    }
}
