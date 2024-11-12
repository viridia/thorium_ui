use std::sync::Arc;

use crate::{
    colors, rounded_corners::RoundedCorners, size::Size, text_styles::UseInheritedTextStyles,
    typography, InheritableFontColor, InheritableFontSize,
};
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    color::Luminance,
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::{IntoSignal, Signal, StyleEntity, StyleHandle, StyleTuple, UiTemplate};
use thorium_ui_headless::{
    focus::{FocusKeyboardInput, KeyboardFocus},
    hover::{Hovering, IsHovering},
    tab_navigation::{IsFocused, KeyboardFocusVisible, TabIndex},
    InteractionDisabled, IsDisabled,
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

pub(crate) fn style_button(ent: &mut EntityCommands) {
    ent.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.justify_content = ui::JustifyContent::Center;
        node.align_items = ui::AlignItems::Center;
        node.align_content = ui::AlignContent::Center;
        node.padding = ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0));
        node.border = ui::UiRect::all(ui::Val::Px(0.0));
    });
    ent.insert(CursorIcon::System(SystemCursorIcon::Pointer));
    ent.insert(InheritableFontColor(colors::FOREGROUND.into()));
}

pub(crate) fn style_button_bg(ent: &mut EntityCommands) {
    ent.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Grid;
        node.position_type = ui::PositionType::Absolute;
        node.left = ui::Val::Px(0.0);
        node.right = ui::Val::Px(0.0);
        node.top = ui::Val::Px(0.0);
        node.bottom = ui::Val::Px(0.0);
    });
}

#[derive(Component)]
pub struct ButtonPressed(pub bool);

#[derive(Component)]
pub(crate) struct ButtonState {
    on_click: Option<SystemId>,
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
    pub children: Arc<dyn Fn(&mut ChildBuilder)>,

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
    pub fn children<V: 'static + Fn(&mut ChildBuilder)>(mut self, children: V) -> Self {
        self.children = Arc::new(children);
        self
    }

    /// Set a child which is a text label.
    pub fn labeled(mut self, label: impl Into<String>) -> Self {
        let s: String = label.into();
        self.children = Arc::new(move |builder| {
            // TODO: Figure out how to avoid the double-copy here.
            builder.spawn((Text::new(s.clone()), UseInheritedTextStyles));
        });
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
            children: Arc::new(|_builder| {}),
            style: StyleHandle::none(),
            on_click: None,
            tab_index: 0,
            corners: RoundedCorners::default(),
            autofocus: false,
            minimal: false,
        }
    }
}

impl UiTemplate for Button {
    fn build(&self, builder: &mut ChildBuilder) {
        let variant = self.variant;

        let corners = self.corners;
        let minimal = self.minimal;

        let size = self.size;
        let on_click = self.on_click;

        let mut button = builder.spawn((Node::default(), Name::new("Button"), Hovering::default()));
        let button_id = button.id();

        button
            .style((
                typography::text_default,
                style_button,
                move |ent: &mut EntityCommands| {
                    ent.entry::<Node>().and_modify(move |mut node| {
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
                    ent.insert(InheritableFontSize(size.font_size()));
                },
                self.style.clone(),
            ))
            // TODO
            // .insert_when(self.disabled, || InteractionDisabled)
            .insert((
                TabIndex(self.tab_index),
                ButtonPressed(false),
                ButtonState { on_click },
                AccessibilityNode::from(NodeBuilder::new(Role::Button)),
            ))
            // .insert_when(self.autofocus, || AutoFocus)
            .with_children(|builder| {
                builder
                    .spawn((Node::default(), Name::new("Button::Background")))
                    .style(style_button_bg)
                    .insert(corners.to_border_radius(self.size.border_radius()))
                    .style_dyn(
                        move |world: DeferredWorld| {
                            if minimal {
                                colors::TRANSPARENT
                            } else {
                                let entity = world.entity(button_id);
                                let pressed = entity
                                    .get::<ButtonPressed>()
                                    .map(|pressed| pressed.0)
                                    .unwrap_or_default();
                                button_bg_color(
                                    variant.get(&world),
                                    world.is_disabled(button_id),
                                    pressed,
                                    world.is_hovering(button_id),
                                )
                            }
                        },
                        |color, ent| {
                            ent.insert(BackgroundColor(color.into()));
                        },
                    )
                    .style_dyn(
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
                    );
                let children = self.children.as_ref();
                (children)(builder);
            });
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

pub(crate) fn button_on_key_event(
    mut trigger: Trigger<FocusKeyboardInput>,
    q_state: Query<(&ButtonState, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((bstate, disabled)) = q_state.get(trigger.entity()) {
        if !disabled {
            let event = &trigger.event().0;
            if !event.repeat
                && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
            {
                if let Some(on_click) = bstate.on_click {
                    trigger.propagate(false);
                    commands.run_system(on_click);
                }
            }
        }
    }
}

pub(crate) fn button_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut q_state: Query<(&ButtonState, &mut ButtonPressed, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((bstate, pressed, disabled)) = q_state.get_mut(trigger.entity()) {
        trigger.propagate(false);
        if pressed.0 && !disabled {
            // println!("Click: {}", pressed.0);
            if let Some(on_click) = bstate.on_click {
                commands.run_system(on_click);
            }
        }
    }
}

pub(crate) fn button_on_pointer_down(
    mut trigger: Trigger<Pointer<Down>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
    mut focus: ResMut<KeyboardFocus>,
    mut focus_visible: ResMut<KeyboardFocusVisible>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.entity()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = true;
            focus.0 = Some(trigger.entity());
            focus_visible.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_up(
    mut trigger: Trigger<Pointer<Up>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.entity()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.entity()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_cancel(
    mut trigger: Trigger<Pointer<Cancel>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.entity()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}
