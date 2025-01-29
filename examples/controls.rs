//! Example of a simple UI layout

use bevy::{
    color::palettes, ecs::world::DeferredWorld, input_focus::tab_navigation::TabGroup, prelude::*,
    ui,
};
use thorium_ui::{
    CreateCallback, CreateMutable, DynChildren, Invoke, Signal, Styles, Template, TemplateContext,
    ThoriumUiCorePlugin,
};
use thorium_ui_controls::{
    colors, Checkbox, ColorGradient, DisclosureToggle, GradientSlider, InheritableFontColor,
    Slider, SpinBox, Swatch, SwatchGrid, ThoriumUiControlsPlugin, UseInheritedTextStyles,
};

fn style_test(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = Display::Flex;
        node.flex_direction = FlexDirection::Column;
        node.position_type = PositionType::Absolute;
        node.left = ui::Val::Px(0.);
        node.top = ui::Val::Px(0.);
        node.right = ui::Val::Px(0.);
        node.bottom = ui::Val::Px(0.);
        node.padding = ui::UiRect::all(Val::Px(3.));
        node.row_gap = ui::Val::Px(4.);
    });
    ec.insert(BackgroundColor(colors::BACKGROUND.into()));
    ec.insert(InheritableFontColor(colors::DIM.into()));
}

fn style_row(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = Display::Flex;
        node.flex_direction = FlexDirection::Row;
        node.align_items = AlignItems::Center;
        node.padding = ui::UiRect::all(Val::Px(3.));
        node.column_gap = ui::Val::Px(4.);
    });
}

fn style_column(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = Display::Flex;
        node.width = ui::Val::Px(300.);
        node.flex_direction = FlexDirection::Column;
        node.align_items = AlignItems::Start;
        node.row_gap = ui::Val::Px(4.);
    });
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ThoriumUiCorePlugin, ThoriumUiControlsPlugin))
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

    commands.spawn((
        Node::default(),
        Styles(style_test),
        UiTargetCamera(camera),
        TabGroup::default(),
        DynChildren::spawn((
            (
                Spawn((Text::new("Swatch"), UseInheritedTextStyles)),
                Spawn((
                    Node::default(),
                    Styles(style_row),
                    DynChildren::spawn((
                        Invoke(Swatch::new(palettes::css::GOLDENROD)),
                        Invoke(Swatch::new(palettes::css::LIME)),
                        Invoke(Swatch::new(palettes::css::RED)),
                        Invoke(Swatch::new(Srgba::NONE)),
                        Invoke(Swatch::new(palettes::css::BLUE).selected(true)),
                    )),
                )),
            ),
            (
                Spawn((Text::new("SwatchGrid"), UseInheritedTextStyles)),
                Spawn((
                    Node::default(),
                    Styles(style_row),
                    DynChildren::spawn(Invoke(SwatchGridDemo)),
                )),
            ),
            (
                Spawn((Text::new("Checkbox"), UseInheritedTextStyles)),
                Spawn((
                    Node::default(),
                    Styles(style_column),
                    DynChildren::spawn(Invoke(CheckboxDemo)),
                )),
            ),
            (
                Spawn((Text::new("Slider"), UseInheritedTextStyles)),
                Spawn((
                    Node::default(),
                    Styles((style_column, |ec: &mut EntityCommands| {
                        ec.entry::<Node>().and_modify(|mut node| {
                            node.align_items = ui::AlignItems::Stretch;
                        });
                    })),
                    DynChildren::spawn(Invoke(SliderDemo)),
                )),
            ),
            (
                Spawn((Text::new("GradientSlider"), UseInheritedTextStyles)),
                Spawn((
                    Node::default(),
                    Styles((style_column, |ec: &mut EntityCommands| {
                        ec.entry::<Node>().and_modify(|mut node| {
                            node.align_items = ui::AlignItems::Stretch;
                        });
                    })),
                    DynChildren::spawn(Invoke(GradientSliderDemo)),
                )),
            ),
            (
                Spawn((Text::new("SpinBox"), UseInheritedTextStyles)),
                Spawn((
                    Node::default(),
                    Styles((style_column, |ec: &mut EntityCommands| {
                        ec.entry::<Node>().and_modify(|mut node| {
                            node.align_items = ui::AlignItems::Stretch;
                        });
                    })),
                    DynChildren::spawn(Invoke(SpinBoxDemo)),
                )),
            ),
            Spawn((Text::new("DisclosureToggle"), UseInheritedTextStyles)),
            Invoke(DisclosureToggleDemo),
        )),
    ));
}

struct SwatchGridDemo;

impl Template for SwatchGridDemo {
    fn build(&self, tc: &mut TemplateContext) {
        let selected_color = tc.create_mutable::<Srgba>(palettes::css::BLUE);
        let on_change_color =
            tc.create_callback_arg(move |color: In<Srgba>, mut world: DeferredWorld| {
                selected_color.set(&mut world, *color);
            });
        tc.invoke(
            SwatchGrid::new(vec![
                palettes::css::BLUE,
                palettes::css::RED,
                palettes::css::GREEN,
                palettes::css::REBECCA_PURPLE,
            ])
            .grid_size(UVec2::new(12, 4))
            .selected(selected_color.signal())
            .on_change(on_change_color),
        );
    }
}

struct CheckboxDemo;

impl Template for CheckboxDemo {
    fn build(&self, tc: &mut TemplateContext) {
        let checked_1 = tc.create_mutable(true);
        let checked_2 = tc.create_mutable(false);
        let on_change_1 =
            tc.create_callback_arg(move |value: In<bool>, mut world: DeferredWorld| {
                checked_1.set(&mut world, *value);
            });
        let on_change_2 =
            tc.create_callback_arg(move |value: In<bool>, mut world: DeferredWorld| {
                checked_2.set(&mut world, *value);
            });
        tc.invoke(
            Checkbox::new()
                .labeled("Checked")
                .checked(checked_1)
                .on_change(on_change_1),
        )
        .invoke(
            Checkbox::new()
                .labeled("Checked (disabled)")
                .checked(checked_1)
                .on_change(on_change_1)
                .disabled(true),
        )
        .invoke(
            Checkbox::new()
                .labeled("Unchecked")
                .checked(checked_2)
                .on_change(on_change_2),
        )
        .invoke(
            Checkbox::new()
                .labeled("Unchecked (disabled)")
                .checked(checked_2)
                .on_change(on_change_2)
                .disabled(true),
        );
    }
}

struct SliderDemo;

impl Template for SliderDemo {
    fn build(&self, tc: &mut TemplateContext) {
        let slider_value = tc.create_mutable::<f32>(50.);
        let on_change_slider =
            tc.create_callback_arg(move |new_value: In<f32>, mut world: DeferredWorld| {
                slider_value.set(&mut world, *new_value);
            });
        tc.spawn((
            Node::default(),
            Styles((style_column, |ec: &mut EntityCommands| {
                ec.entry::<Node>().and_modify(|mut node| {
                    node.align_items = ui::AlignItems::Stretch;
                });
            })),
            DynChildren::spawn((
                Invoke(
                    Slider::new()
                        .min(0.)
                        .max(100.)
                        .value(slider_value)
                        .on_change(on_change_slider),
                ),
                Invoke(
                    Slider::new()
                        .min(0.)
                        .max(100.)
                        .value(slider_value)
                        .label("Value:")
                        .on_change(on_change_slider),
                ),
            )),
        ));
    }
}

struct GradientSliderDemo;

impl Template for GradientSliderDemo {
    fn build(&self, tc: &mut TemplateContext) {
        tc.spawn((Text::new("GradientSlider"), UseInheritedTextStyles));
        let red = tc.create_mutable::<f32>(128.);
        let on_change_red =
            tc.create_callback_arg(move |new_value: In<f32>, mut world: DeferredWorld| {
                red.set(&mut world, *new_value);
            });
        tc.invoke(
            GradientSlider::new()
                .gradient(Signal::Constant(ColorGradient::new(&[
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    Srgba::new(1.0, 0.0, 0.0, 1.0),
                ])))
                .min(0.)
                .max(255.)
                .value(red)
                // .style(style_slider)
                .precision(1)
                .on_change(on_change_red),
        );
    }
}

struct SpinBoxDemo;

impl Template for SpinBoxDemo {
    fn build(&self, tc: &mut TemplateContext) {
        let spinbox_value = tc.create_mutable::<f32>(50.);
        let on_change_spinbox =
            tc.create_callback_arg(move |new_value: In<f32>, mut world: DeferredWorld| {
                spinbox_value.set(&mut world, *new_value);
            });
        tc.invoke(
            SpinBox::new()
                .min(0.)
                .max(100.)
                .value(spinbox_value)
                .on_change(on_change_spinbox),
        );
    }
}

struct DisclosureToggleDemo;

impl Template for DisclosureToggleDemo {
    fn build(&self, tc: &mut TemplateContext) {
        let expanded = tc.create_mutable(false);
        let on_change = tc.create_callback_arg(move |value: In<bool>, mut world: DeferredWorld| {
            expanded.set(&mut world, *value);
        });

        tc.spawn((
            Node::default(),
            DynChildren::spawn(Invoke(
                DisclosureToggle::new()
                    .expanded(expanded)
                    .on_change(on_change),
            )),
        ));
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
