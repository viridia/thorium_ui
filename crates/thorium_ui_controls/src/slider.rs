use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use thorium_ui_core::*;
use thorium_ui_headless::{hover::Hovering, CoreSlider, ValueChange};

use crate::{
    colors, materials::SliderRectMaterial, rounded_corners::RoundedCorners, typography, IconButton,
    InheritableFontColor, InheritableFontSize, Spacer, UseInheritedTextStyles,
};

fn style_slider(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.min_width = ui::Val::Px(64.);
        node.height = ui::Val::Px(20.);
    });
}

fn style_overlay(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.align_items = ui::AlignItems::Center;
        node.position_type = ui::PositionType::Absolute;
        node.left = ui::Val::Px(0.);
        node.top = ui::Val::Px(0.);
        node.bottom = ui::Val::Px(0.);
        node.right = ui::Val::Px(0.);
    });
    ec.insert(CursorIcon::System(SystemCursorIcon::ColResize));
}

fn style_slider_button(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.height = ui::Val::Px(20.);
        node.padding = ui::UiRect::all(ui::Val::Px(0.));
        node.max_width = ui::Val::Px(12.);
        node.flex_grow = 0.2;
    });
}

fn style_label(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.align_items = ui::AlignItems::Center;
        node.justify_content = ui::JustifyContent::Center;
        node.height = ui::Val::Percent(100.);
        node.padding = ui::UiRect::axes(ui::Val::Px(6.), ui::Val::Px(0.));
        node.flex_grow = 1.;
    });
    ec.insert(InheritableFontSize(14.));
    ec.insert(InheritableFontColor(colors::FOREGROUND.into()));
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
    fn build(&self, builder: &mut ChildSpawnerCommands) {
        let mut slider = builder.spawn((
            MaterialNode::<SliderRectMaterial>::default(),
            Name::new("Slider"),
            Hovering::default(),
        ));

        let min = self.min;
        let max = self.max;
        let value = self.value;
        let precision = self.precision;
        let label = self.label.clone();
        let step = self.step;
        let on_change = self.on_change;

        let dec_click =
            slider.create_callback(move |world: DeferredWorld, mut commands: Commands| {
                let min = min.get(&world);
                let max = max.get(&world);
                let next_value = (value.get(&world) - step).clamp(min, max);
                if let Some(on_change) = on_change {
                    commands.run_system_with(on_change, next_value);
                }
            });

        let inc_click =
            slider.create_callback(move |world: DeferredWorld, mut commands: Commands| {
                let min = min.get(&world);
                let max = max.get(&world);
                let next_value = (value.get(&world) + step).clamp(min, max);
                if let Some(on_change) = on_change {
                    commands.run_system_with(on_change, next_value);
                }
            });

        slider
            .style((typography::text_default, style_slider, self.style.clone()))
            .attach(MutateDyn::new(
                move |world: DeferredWorld| (value.get(&world), min.get(&world), max.get(&world)),
                |(value, min, max), ent| {
                    let core_slider = CoreSlider { value, min, max };
                    let material_handle = ent
                        .get::<MaterialNode<SliderRectMaterial>>()
                        .unwrap()
                        .0
                        .clone();
                    let mut ui_materials = unsafe {
                        ent.world_mut()
                            .get_resource_mut::<Assets<SliderRectMaterial>>()
                            .unwrap()
                    };
                    if material_handle == Handle::default() {
                        let material = ui_materials.add(SliderRectMaterial {
                            color_lo: LinearRgba::from(colors::U1).to_vec4(),
                            color_hi: LinearRgba::from(colors::U3).to_vec4(),
                            value: Vec4::new(core_slider.thumb_position(), 0., 0., 0.),
                            radius: RoundedCorners::All.to_vec(4.),
                        });
                        ent.insert((core_slider, MaterialNode(material)));
                    } else {
                        ui_materials.get_mut(&material_handle).unwrap().value.x =
                            core_slider.thumb_position();
                        ent.insert(core_slider);
                    }
                },
            ))
            .observe(
                move |mut trigger: Trigger<ValueChange<f32>>,
                      world: DeferredWorld,
                      mut commands: Commands| {
                    trigger.propagate(false);
                    let event = trigger.event();
                    let rounding = f32::powi(10., precision as i32);
                    let value = value.get(&world);
                    let new_value = ((event.0 * rounding).round() / rounding)
                        .clamp(min.get(&world), max.get(&world));
                    if value != new_value {
                        if let Some(on_change) = on_change {
                            commands.run_system_with(on_change, new_value);
                        }
                    }
                },
            )
            .with_children(|builder| {
                let dec_disabled = builder.create_memo(
                    move |world: DeferredWorld| value.get(&world) <= min.get(&world),
                    false,
                );
                let inc_disabled = builder.create_memo(
                    move |world: DeferredWorld| value.get(&world) >= max.get(&world),
                    false,
                );
                builder
                    .spawn((Node::default(), Name::new("Slider::Overlay")))
                    .style(style_overlay)
                    .with_children(move |builder| {
                        builder.cond(
                            move || true,
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
                            .with_children(|builder| {
                                if let Some(label) = label {
                                    builder.spawn((Text::new(label), UseInheritedTextStyles));
                                    builder.invoke(Spacer);
                                }
                                builder
                                    .spawn((Text::new(""), UseInheritedTextStyles))
                                    .attach(MutateDyn::new(
                                        move |world: DeferredWorld| value.get(&world),
                                        move |value, ent| {
                                            ent.entry::<Text>().and_modify(|mut text| {
                                                text.0 = format!("{:.*}", precision, value);
                                            });
                                        },
                                    ));
                            });
                        builder.cond(
                            move || true,
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
