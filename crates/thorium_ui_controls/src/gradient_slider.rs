use bevy::{
    color::Srgba,
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
};
use thorium_ui_core::{
    CreateMemo, IntoSignal, MutateDyn, Signal, StyleDyn, StyleHandle, StyleTuple, Styles,
    UiTemplate,
};
use thorium_ui_headless::{hover::Hovering, CoreSlider, ValueChange};

use crate::{image_handle::UiImageHandle, materials::GradientRectMaterial};

const THUMB_WIDTH: f32 = 20.;

/// Struct representing a sequence of color stops, evenly spaced. Up to 8 stops are supported.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorGradient {
    /// Number of color stops.
    pub num_colors: usize,

    /// Array of color stops.
    pub colors: [Srgba; 8],
}

impl ColorGradient {
    /// Construct a new color gradient from an array of colors.
    pub fn new(colors: &[Srgba]) -> Self {
        assert!(colors.len() <= 8);
        let mut result = Self {
            num_colors: colors.len(),
            colors: [Srgba::default(); 8],
        };
        for (i, color) in colors.iter().enumerate() {
            result.colors[i] = *color;
        }
        result
    }

    /// Return the first color in the gradient, if any.
    pub fn first(&self) -> Option<Srgba> {
        if self.num_colors > 0 {
            Some(self.colors[0])
        } else {
            None
        }
    }

    /// Return the last color in the gradient, if any.
    pub fn last(&self) -> Option<Srgba> {
        if self.num_colors > 0 {
            Some(self.colors[self.num_colors - 1])
        } else {
            None
        }
    }

    /// Return the number of color stops in the gradient.
    pub fn len(&self) -> usize {
        self.num_colors
    }

    /// Check if the gradient is empty.
    pub fn is_empty(&self) -> bool {
        self.num_colors == 0
    }
}

impl Default for ColorGradient {
    fn default() -> Self {
        Self {
            num_colors: 1,
            colors: [Srgba::BLACK; 8],
        }
    }
}

fn style_slider(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.align_items = ui::AlignItems::Stretch;
        node.height = ui::Val::Px(THUMB_WIDTH);
        node.min_width = ui::Val::Px(32.);
    });
}

fn style_gradient(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.flex_grow = 1.;
        node.position_type = ui::PositionType::Absolute;
        node.top = ui::Val::Px(4.);
        node.bottom = ui::Val::Px(4.);
        node.left = ui::Val::Px(2.);
        node.right = ui::Val::Px(2.);
    });
}

fn style_track(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.position_type = ui::PositionType::Absolute;
        node.top = ui::Val::Px(1.);
        node.bottom = ui::Val::Px(1.);
        node.left = ui::Val::Px(1.);
        node.right = ui::Val::Px(THUMB_WIDTH + 1.);
    });
}

fn style_thumb(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.position_type = ui::PositionType::Absolute;
        node.top = ui::Val::Px(-1.);
        // node.bottom = ui::Val::Px(0.);
        node.width = ui::Val::Px(THUMB_WIDTH);
        node.height = ui::Val::Px(THUMB_WIDTH);
    });
}

/// Horizontal slider widget that displays a gradient bar and a draggable button.
pub struct GradientSlider {
    /// Gradient to display.
    pub gradient: Signal<ColorGradient>,

    /// Current slider value.
    pub value: Signal<f32>,

    /// Minimum slider value.
    pub min: Signal<f32>,

    /// Maximum slider value.
    pub max: Signal<f32>,

    /// Number of decimal places to round to (0 = integer).
    pub precision: usize,

    /// Whether the slider is disabled.
    pub disabled: Signal<bool>,

    /// Style handle for slider root element.
    pub style: StyleHandle,

    /// Callback called when value changes
    pub on_change: Option<SystemId<In<f32>>>,
}

impl GradientSlider {
    /// Create a new gradient slider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the gradient to display.
    pub fn gradient(mut self, gradient: impl IntoSignal<ColorGradient>) -> Self {
        self.gradient = gradient.into_signal();
        self
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

    /// Set whether the slider is disabled.
    pub fn disabled(mut self, disabled: impl IntoSignal<bool>) -> Self {
        self.disabled = disabled.into_signal();
        self
    }

    /// Set the style handle for the slider root element.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when the value changes.
    pub fn on_change(mut self, on_change: SystemId<In<f32>>) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

impl Default for GradientSlider {
    fn default() -> Self {
        Self {
            gradient: Signal::Constant(ColorGradient::default()),
            value: Signal::Constant(0.),
            min: Signal::Constant(0.),
            max: Signal::Constant(1.),
            precision: 0,
            disabled: Signal::Constant(false),
            style: StyleHandle::default(),
            on_change: None,
        }
    }
}

impl UiTemplate for GradientSlider {
    fn build(&self, builder: &mut ChildSpawnerCommands) {
        // This should really be an effect.
        let color_stops: Signal<(usize, [Vec4; 8])> = {
            let gradient = self.gradient;
            builder
                .create_memo(
                    move |world: DeferredWorld| {
                        gradient.map(&world, |g| {
                            let mut result: [Vec4; 8] = [Vec4::default(); 8];
                            let num_color_stops = g.len();
                            for (i, color) in g.colors[0..num_color_stops].iter().enumerate() {
                                // Note that we do *not* convert to linear here, because interpolating
                                // linear looks bad. That gets done in the shader.
                                result[i] =
                                    Vec4::new(color.red, color.green, color.blue, color.alpha);
                            }
                            (g.len(), result)
                        })
                    },
                    (0, [Vec4::default(); 8]),
                )
                .into_signal()
        };

        let mut slider = builder.spawn((
            Node::default(),
            Name::new("GradientSlider"),
            Hovering::default(),
            Styles((style_slider, self.style.clone())),
        ));
        let slider_id = slider.id();

        let min = self.min;
        let max = self.max;
        let value = self.value;
        let precision = self.precision;
        let on_change = self.on_change;

        slider
            .insert(MutateDyn::new(
                move |world: DeferredWorld| (value.get(&world), min.get(&world), max.get(&world)),
                |(value, min, max), ent| {
                    ent.insert(CoreSlider::new(value, min, max));
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
            .observe(
                move |mut trigger: Trigger<Pointer<Pressed>>,
                      world: DeferredWorld,
                      mut commands: Commands| {
                    trigger.propagate(false);
                    let min = min.get(&world);
                    let max = max.get(&world);
                    let hit_x = trigger.event().pointer_location.position.x;
                    let ent = world.entity(slider_id);
                    let node = ent.get::<ComputedNode>();
                    let transform = ent.get::<GlobalTransform>();
                    if let (Some(node), Some(transform)) = (node, transform) {
                        // If not clicking on thumb, then snap thumb to new location.
                        let rect =
                            Rect::from_center_size(transform.translation().xy(), node.size());
                        let slider_width = rect.width() - THUMB_WIDTH;
                        let range = max - min;
                        let pointer_pos = hit_x - rect.min.x - THUMB_WIDTH / 2.;
                        let thumb_pos =
                            value.get(&world) - min * slider_width / range + THUMB_WIDTH / 2.;
                        if range > 0. && (pointer_pos - thumb_pos).abs() >= THUMB_WIDTH / 2. {
                            let new_value = min + (pointer_pos * range) / slider_width;
                            if let Some(on_change) = on_change {
                                commands.run_system_with(on_change, new_value.clamp(min, max));
                            }
                        };
                    }
                },
            )
            .with_children(|builder| {
                builder.spawn((
                    MaterialNode::<GradientRectMaterial>::default(),
                    Styles(style_gradient),
                    MutateDyn::new(
                        move |world: DeferredWorld| color_stops.get(&world),
                        |(num_color_stops, color_stops), ent| {
                            let material_handle = ent
                                .get::<MaterialNode<GradientRectMaterial>>()
                                .unwrap()
                                .0
                                .clone();
                            let mut ui_materials = unsafe {
                                ent.world_mut()
                                    .get_resource_mut::<Assets<GradientRectMaterial>>()
                                    .unwrap()
                            };
                            if material_handle == Handle::default() {
                                let material = ui_materials.add(GradientRectMaterial {
                                    color_stops,
                                    num_color_stops: IVec4::new(num_color_stops as i32, 0, 0, 0),
                                    cap_size: THUMB_WIDTH * 0.5,
                                });
                                ent.insert(MaterialNode(material));
                            } else {
                                let material = ui_materials.get_mut(&material_handle).unwrap();
                                material.num_color_stops.x = num_color_stops as i32;
                                material.color_stops = color_stops;
                            }
                        },
                    ),
                ));
                builder.spawn((
                    Node::default(),
                    Name::new("GradientSlider::Track"),
                    Styles(style_track),
                    children![(
                        ImageNode {
                            color: Srgba::WHITE.into(),
                            ..default()
                        },
                        UiImageHandle(
                            "embedded://thorium_ui_controls/assets/icons/gradient_thumb.png".into(),
                        ),
                        Name::new("GradientSlider::Thumb"),
                        Styles(style_thumb),
                        StyleDyn::new(
                            move |world: DeferredWorld| {
                                let min = min.get(&world);
                                let max = max.get(&world);
                                let value = value.get(&world);
                                CoreSlider::new(value, min, max).thumb_position()
                            },
                            |percent, ec| {
                                ec.entry::<Node>().and_modify(move |mut node| {
                                    node.left = ui::Val::Percent(percent * 100.);
                                });
                            },
                        ),
                    )],
                ));
            });
    }
}
