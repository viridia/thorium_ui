use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::{
    CreateCallback, CreateCond, CreateMemo, CreateMutable, EntitEffect, IntoSignal,
    InvokeUiTemplate, Signal, StyleEntity, StyleHandle, StyleTuple, UiTemplate,
};

use crate::{
    colors, rounded_corners::RoundedCorners, typography, InheritableFontColor, InheritableFontSize,
    UseInheritedTextStyles,
};

use super::IconButton;

#[derive(Clone, PartialEq, Default, Copy)]
enum DragType {
    #[default]
    None = 0,
    Dragging,
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: DragType,
    offset: f32,
    was_dragged: bool,
}

fn style_spinbox(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.align_items = ui::AlignItems::Center;
        node.min_width = ui::Val::Px(64.);
        node.height = ui::Val::Px(20.);
    });
    ec.insert(BackgroundColor(colors::U1.into()));
    ec.insert(BorderRadius::all(ui::Val::Px(5.)));
}

fn style_spinbox_label(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.flex_grow = 1.;
        node.justify_content = ui::JustifyContent::Center;
        node.align_items = ui::AlignItems::Center;
        // node.height = ui::Val::Percent(100.);
        node.padding = ui::UiRect::axes(ui::Val::Px(3.0), ui::Val::Px(0.));
        node.overflow = ui::Overflow {
            x: ui::OverflowAxis::Hidden,
            y: ui::OverflowAxis::Visible,
        };
    });
    ec.insert(InheritableFontSize(14.));
    ec.insert(InheritableFontColor(colors::FOREGROUND.into()));
    ec.insert(CursorIcon::System(SystemCursorIcon::ColResize));
}

fn style_spinbox_button(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.height = ui::Val::Px(20.);
        node.padding = ui::UiRect::all(ui::Val::Px(0.));
        node.max_width = ui::Val::Px(12.);
        node.flex_grow = 0.2;
    });
}

/// A numeric spinbox. This is a widget that allows the user to input a number by typing, using
/// arrow buttons, or dragging. It is preferred over a slider in two cases:
/// * The range of values is large or unbounded, making it difficult to select a specific value
///   with a slider.
/// * There is limited horizontal space available.
pub struct SpinBox {
    /// Current slider value.
    pub value: Signal<f32>,

    /// Minimum slider value.
    pub min: Signal<f32>,

    /// Maximum slider value.
    pub max: Signal<f32>,

    /// Number of decimal places to round to (0 = integer).
    pub precision: usize,

    /// Amount to increment when using arrow buttons.
    pub step: f32,

    /// Whether the slider is disabled.
    pub disabled: Signal<bool>,

    /// Signal which returns the value formatted as a string. It `None`, then a default
    /// formatter will be used.
    pub formatted_value: Option<Signal<String>>,

    /// Style handle for slider root element.
    pub style: StyleHandle,

    /// Callback called when value changes
    pub on_change: Option<SystemId<In<f32>>>,
}

impl SpinBox {
    /// Create a new spinbox.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current spinbox value.
    pub fn value(mut self, value: impl IntoSignal<f32>) -> Self {
        self.value = value.into_signal();
        self
    }

    /// Set the minimum spinbox value.
    pub fn min(mut self, min: impl IntoSignal<f32>) -> Self {
        self.min = min.into_signal();
        self
    }

    /// Set the maximum spinbox value.
    pub fn max(mut self, max: impl IntoSignal<f32>) -> Self {
        self.max = max.into_signal();
        self
    }

    /// Set the number of decimal places to round to (0 = integer).
    pub fn precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    /// Set the amount to increment when using arrow buttons.
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set whether the spinbox is disabled.
    pub fn disabled(mut self, disabled: impl IntoSignal<bool>) -> Self {
        self.disabled = disabled.into_signal();
        self
    }

    /// Set the signal which returns the value formatted as a string. If `None`, then a default
    /// formatter will be used.
    pub fn formatted_value(mut self, formatted_value: impl IntoSignal<String>) -> Self {
        self.formatted_value = Some(formatted_value.into_signal());
        self
    }

    /// Set the style handle for the spinbox root element.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when value changes.
    pub fn on_change(mut self, on_change: SystemId<In<f32>>) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

impl Default for SpinBox {
    fn default() -> Self {
        Self {
            value: Signal::Constant(0.),
            min: Signal::Constant(f32::MIN),
            max: Signal::Constant(f32::MAX),
            precision: 0,
            step: 1.,
            disabled: Signal::Constant(false),
            formatted_value: None,
            style: StyleHandle::default(),
            on_change: None,
        }
    }
}

impl UiTemplate for SpinBox {
    fn build(&self, builder: &mut ChildBuilder) {
        let drag_state = builder.create_mutable::<DragState>(DragState::default());
        let mut spinbox = builder.spawn((Node::default(), Name::new("Spinbox")));
        // let show_buttons = builder.create_derived(move |rcx| {
        //     // Show buttons when spinbox is wide enough.
        //     let node = rcx.read_component::<ComputedNode>(spinbox_id).unwrap();
        //     node.size().x >= 48.
        // });

        let min = self.min;
        let max = self.max;
        let value = self.value;
        let precision = self.precision;
        let step = self.step;
        let on_change = self.on_change;

        let dec_click =
            spinbox.create_callback(move |world: DeferredWorld, mut commands: Commands| {
                let min = min.get(&world);
                let max = max.get(&world);
                let next_value = (value.get(&world) - step).clamp(min, max);
                if let Some(on_change) = on_change {
                    commands.run_system_with_input(on_change, next_value);
                }
            });

        let inc_click =
            spinbox.create_callback(move |world: DeferredWorld, mut commands: Commands| {
                let min = min.get(&world);
                let max = max.get(&world);
                let next_value = (value.get(&world) + step).clamp(min, max);
                if let Some(on_change) = on_change {
                    commands.run_system_with_input(on_change, next_value);
                }
            });

        spinbox
            .style((style_spinbox, self.style.clone()))
            .with_children(|builder| {
                let dec_disabled = builder.create_memo(
                    move |world: DeferredWorld| value.get(&world) <= min.get(&world),
                    false,
                );
                let inc_disabled = builder.create_memo(
                    move |world: DeferredWorld| value.get(&world) >= max.get(&world),
                    false,
                );

                builder.cond(
                    move || true,
                    move |builder| {
                        builder.invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .corners(RoundedCorners::Left)
                            .style(style_spinbox_button)
                            .minimal(true)
                            .disabled(dec_disabled)
                            .on_click(dec_click),
                        );
                    },
                    |_| {},
                );

                builder
                    .spawn((Node::default(), Name::new("SpinBox::Label")))
                    .style((typography::text_default, style_spinbox_label))
                    .observe(
                        move |mut trigger: Trigger<Pointer<DragStart>>,
                              mut world: DeferredWorld| {
                            trigger.propagate(false);
                            let offset = value.get(&world);
                            drag_state.set(
                                &mut world,
                                DragState {
                                    dragging: DragType::Dragging,
                                    offset,
                                    was_dragged: false,
                                },
                            );
                        },
                    )
                    .observe(
                        move |mut trigger: Trigger<Pointer<DragEnd>>, mut world: DeferredWorld| {
                            trigger.propagate(false);
                            let offset = value.get(&world);
                            let ds = drag_state.get(&world);
                            if ds.dragging == DragType::Dragging {
                                drag_state.set(
                                    &mut world,
                                    DragState {
                                        dragging: DragType::None,
                                        offset,
                                        was_dragged: false,
                                    },
                                );
                            }
                        },
                    )
                    .observe(
                        move |mut trigger: Trigger<Pointer<Drag>>,
                              mut world: DeferredWorld,
                              mut commands: Commands| {
                            trigger.propagate(false);
                            let ds = drag_state.get(&world);
                            if ds.dragging == DragType::Dragging {
                                let min = min.get(&world);
                                let max = max.get(&world);
                                let event = trigger.event();
                                let new_value = ds.offset
                                    + ((event.distance.x - event.distance.y) * 0.1 * step);
                                let rounding = f32::powi(10., precision as i32);
                                let value = value.get(&world);
                                let new_value = (new_value * rounding).round() / rounding;
                                if value != new_value {
                                    if !ds.was_dragged {
                                        drag_state.set(
                                            &mut world,
                                            DragState {
                                                was_dragged: true,
                                                ..ds
                                            },
                                        );
                                    }
                                    if let Some(on_change) = on_change {
                                        commands.run_system_with_input(
                                            on_change,
                                            new_value.clamp(min, max),
                                        );
                                    }
                                }
                            }
                        },
                    )
                    .with_children(|builder| {
                        builder
                            .spawn((Text::new(""), UseInheritedTextStyles))
                            .effect(
                                move |world: DeferredWorld| value.get(&world),
                                move |value, ent| {
                                    ent.entry::<Text>().and_modify(|mut text| {
                                        text.0 = format!("{:.*}", precision, value);
                                    });
                                },
                            );
                    });

                builder.cond(
                    move || true,
                    move |builder| {
                        builder.invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_right.png",
                            )
                            .corners(RoundedCorners::Right)
                            .style(style_spinbox_button)
                            .minimal(true)
                            .disabled(inc_disabled)
                            .on_click(inc_click),
                        );
                    },
                    |_| {},
                );
            });
    }
}
