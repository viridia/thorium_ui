use bevy::{
    color::LinearRgba,
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::*;

use crate::{colors, materials::SliderRectMaterial, typography};

fn style_slider(ss: &mut EntityCommands) {
    // ss.min_width(64).height(20);
}

fn style_overlay(ss: &mut EntityCommands) {
    // ss.display(ui::Display::Flex)
    //     .flex_direction(ui::FlexDirection::Row)
    //     .align_items(ui::AlignItems::Center)
    //     .position(ui::PositionType::Absolute)
    //     .left(0)
    //     .top(0)
    //     .bottom(0)
    //     .right(0)
    //     .cursor(CursorIcon::System(SystemCursorIcon::ColResize));
}

fn style_slider_button(ss: &mut EntityCommands) {
    // ss.height(20.).padding(0).max_width(12).flex_grow(0.2);
}

fn style_label(ss: &mut EntityCommands) {
    // ss.flex_grow(1.)
    //     .display(ui::Display::Flex)
    //     .flex_direction(ui::FlexDirection::Row)
    //     .align_items(ui::AlignItems::Center)
    //     .justify_content(ui::JustifyContent::Center)
    //     .height(ui::Val::Percent(100.))
    //     .font_size(14)
    //     .padding((6, 0))
    //     .color(colors::FOREGROUND);
}

/// Horizontal slider widget
pub struct Slider {
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

    /// Optional label to be displayed inside the slider.
    pub label: Option<String>,

    /// Style handle for slider root element.
    pub style: StyleHandle,

    /// Callback called when value changes
    pub on_change: Option<SystemId<In<f32>>>,
}

impl Slider {
    /// Create a new slider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current slider value.
    pub fn value(mut self, value: impl IntoSignal<f32>) -> Self {
        self.value = value.into_signal();
        self
    }

    /// Set the minimum slider value.
    pub fn min(mut self, min: impl IntoSignal<f32>) -> Self {
        self.min = min.into_signal();
        self
    }

    /// Set the maximum slider value.
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

    /// Set whether the slider is disabled.
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

    /// Set the optional label to be displayed inside the slider.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the style handle for the slider root element.
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

impl Default for Slider {
    fn default() -> Self {
        Self {
            value: Signal::Constant(0.),
            min: Signal::Constant(0.),
            max: Signal::Constant(1.),
            precision: 0,
            step: 1.,
            disabled: Signal::Constant(false),
            formatted_value: None,
            style: StyleHandle::default(),
            label: None,
            on_change: None,
        }
    }
}

impl UiTemplate for Slider {
    fn build(&self, builder: &mut ChildBuilder) {
        let slider_id = builder
            .spawn((
                MaterialNode::<SliderRectMaterial>::default(),
                Name::new("Slider"),
            ))
            .id();
        // let drag_state = builder.create_mutable::<DragState>(DragState::default());
        let show_buttons = Signal::Constant(true);

        // Pain point: Need to capture all props for closures.
        let min = self.min;
        let max = self.max;
        let value = self.value;
        let precision = self.precision;
        let label = self.label.clone();
        let step = self.step;
        let on_change = self.on_change;

        let mut ui_materials = builder
            .world_mut()
            .get_resource_mut::<Assets<SliderRectMaterial>>()
            .unwrap();
        let material = ui_materials.add(SliderRectMaterial {
            color_lo: LinearRgba::from(colors::U1).to_vec4(),
            color_hi: LinearRgba::from(colors::U3).to_vec4(),
            value: Vec4::new(0.5, 0., 0., 0.),
            radius: RoundedCorners::All.to_vec(4.),
        });
        let material_id = material.id();

        // Effect to update the material with the slider position.
        builder.create_effect(move |ecx| {
            let min = min.get(ecx);
            let max = max.get(ecx);
            let value = value.get(ecx);
            let pos = if max > min {
                (value - min) / (max - min)
            } else {
                0.
            };

            let mut ui_materials = ecx
                .world_mut()
                .get_resource_mut::<Assets<SliderRectMaterial>>()
                .unwrap();
            let material = ui_materials.get_mut(material_id).unwrap();
            material.value.x = pos;
        });

        builder
            .entity_mut(slider_id)
            .styles((typography::text_default, style_slider, self.style.clone()))
            .insert(MaterialNode(material.clone()))
            .effect(
                move |rcx| CoreSlider::new(value.get(rcx), min.get(rcx), max.get(rcx)),
                |slider, ent| {
                    ent.insert(slider);
                },
            )
            .observe(
                move |mut trigger: Trigger<ValueChange<f32>>, mut world: DeferredWorld| {
                    trigger.propagate(false);
                    let event = trigger.event();
                    let rounding = f32::powi(10., precision as i32);
                    let value = value.get(&world);
                    let new_value = ((event.0 * rounding).round() / rounding)
                        .clamp(min.get(&world), max.get(&world));
                    if value != new_value {
                        if let Some(on_change) = on_change {
                            world.run_callback(on_change, new_value);
                        }
                    }
                },
            )
            .create_children(|builder| {
                let dec_disabled =
                    builder.create_derived(move |rcx| value.get(rcx) <= min.get(rcx));
                let dec_click =
                    builder.create_callback(move |_in: In<()>, mut world: DeferredWorld| {
                        let min = min.get(&world);
                        let max = max.get(&world);
                        let next_value = (value.get(&world) - step).clamp(min, max);
                        if let Some(on_change) = on_change {
                            world.run_callback(on_change, next_value);
                        }
                    });
                let inc_disabled =
                    builder.create_derived(move |rcx| value.get(rcx) >= max.get(rcx));
                let inc_click =
                    builder.create_callback(move |_in: In<()>, mut world: DeferredWorld| {
                        let min = min.get(&world);
                        let max = max.get(&world);
                        let next_value = (value.get(&world) + step).clamp(min, max);
                        if let Some(on_change) = on_change {
                            world.run_callback(on_change, next_value);
                        }
                    });
                builder
                    .spawn((Node::default(), Name::new("Slider::Overlay")))
                    .style(style_overlay)
                    .create_children(move |builder| {
                        builder.cond(
                            show_buttons,
                            move |builder| {
                                builder.invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .corners(RoundedCorners::Left)
                            .style(style_slider_button)
                            .minimal(true)
                            .disabled(dec_disabled)
                            .on_click(dec_click));
                            },
                            |_| {},
                        );
                        builder
                            .spawn(Node::default())
                            .style(style_label)
                            .create_children(|builder| {
                                if let Some(label) = label {
                                    builder.text(label);
                                    builder.invoke(Spacer);
                                }
                                builder.text_computed({
                                    move |rcx| {
                                        let value = value.get(rcx);
                                        format!("{:.*}", precision, value)
                                    }
                                });
                            });
                        builder.cond(
                            show_buttons,
                            move |builder| {
                                builder.invoke(
                                IconButton::new(
                                    "embedded://thorium_ui_controls/assets/icons/chevron_right.png",
                                )
                                .corners(RoundedCorners::Right)
                                .style(style_slider_button)
                                .minimal(true)
                                .disabled(inc_disabled)
                                .on_click(inc_click));
                            },
                            |_| {},
                        );
                    });
            });
    }
}
