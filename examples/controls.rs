//! Example of a simple UI layout

use bevy::{color::palettes, ecs::world::DeferredWorld, prelude::*, ui};
use thorium_ui::{
    tab_navigation::{handle_tab_navigation, TabGroup},
    CreateCallback, CreateMutable, InvokeUiTemplate, StyleEntity, ThoriumUiCorePlugin,
};
use thorium_ui_controls::{
    colors, InheritableFontColor, Slider, ThoriumUiControlsPlugin, UseInheritedTextStyles,
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

    commands
        .spawn(Node::default())
        .insert((TargetCamera(camera), TabGroup::default()))
        .observe(handle_tab_navigation)
        .style(style_test)
        .with_children(|builder| {
            builder.spawn((Text::new("Swatch"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|_builder| {
                    // builder
                    //     .invoke(Swatch::new(palettes::css::GOLDENROD))
                    //     .invoke(Swatch::new(palettes::css::LIME))
                    //     .invoke(Swatch::new(palettes::css::RED))
                    //     .invoke(Swatch::new(Srgba::NONE))
                    //     .invoke(Swatch::new(palettes::css::BLUE).selected(true));
                });

            // builder.text("SwatchGrid");
            // builder
            //     .spawn(Node::default())
            //     .style(style_row)
            //     .with_children(|builder| {
            //         let selected = builder.create_mutable::<Srgba>(palettes::css::BLUE);
            //         let on_change = builder.create_callback(
            //             move |color: In<Srgba>, mut world: DeferredWorld| {
            //                 selected.set(&mut world, *color);
            //             },
            //         );
            //         builder.invoke(
            //             SwatchGrid::new(vec![
            //                 palettes::css::BLUE,
            //                 palettes::css::RED,
            //                 palettes::css::GREEN,
            //                 palettes::css::REBECCA_PURPLE,
            //             ])
            //             .grid_size(UVec2::new(12, 4))
            //             .selected(selected.signal())
            //             .on_change(on_change),
            //         );
            //     });

            // builder.text("Checkbox");
            // builder
            //     .spawn(Node::default())
            //     .style(style_column)
            //     .with_children(|builder| {
            //         let checked_1 = builder.create_mutable(true);
            //         let checked_2 = builder.create_mutable(false);
            //         let on_change_1 = builder.create_callback(
            //             move |value: In<bool>, mut world: DeferredWorld| {
            //                 checked_1.set(&mut world, *value);
            //             },
            //         );
            //         let on_change_2 = builder.create_callback(
            //             move |value: In<bool>, mut world: DeferredWorld| {
            //                 checked_2.set(&mut world, *value);
            //             },
            //         );
            //         builder
            //             .invoke(
            //                 Checkbox::new()
            //                     .labeled("Checked")
            //                     .checked(checked_1)
            //                     .on_change(on_change_1),
            //             )
            //             .invoke(
            //                 Checkbox::new()
            //                     .labeled("Checked (disabled)")
            //                     .checked(checked_1)
            //                     .on_change(on_change_1)
            //                     .disabled(true),
            //             )
            //             .invoke(
            //                 Checkbox::new()
            //                     .labeled("Unchecked")
            //                     .checked(checked_2)
            //                     .on_change(on_change_2),
            //             )
            //             .invoke(
            //                 Checkbox::new()
            //                     .labeled("Unchecked (disabled)")
            //                     .checked(checked_2)
            //                     .on_change(on_change_2)
            //                     .disabled(true),
            //             );
            //     });
            builder.spawn((Text::new("Slider"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style((style_column, |ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(|mut node| {
                        node.align_items = ui::AlignItems::Stretch;
                    });
                }))
                .with_children(|builder| {
                    let value = builder.create_mutable::<f32>(50.);
                    let on_change = builder.create_callback_arg(
                        move |new_value: In<f32>, mut world: DeferredWorld| {
                            value.set(&mut world, *new_value);
                        },
                    );
                    builder
                        .invoke(
                            Slider::new()
                                .min(0.)
                                .max(100.)
                                .value(value)
                                .on_change(on_change),
                        )
                        .invoke(
                            Slider::new()
                                .min(0.)
                                .max(100.)
                                .value(value)
                                .label("Value:")
                                .on_change(on_change),
                        );
                });

            // builder.text("GradientSlider");
            // builder
            //     .spawn(Node::default())
            //     .styles((style_column, |sb: &mut StyleBuilder| {
            //         sb.align_items(ui::AlignItems::Stretch);
            //     }))
            //     .with_children(|builder| {
            //         let red = builder.create_mutable::<f32>(128.);
            //         let red_gradient = builder.create_derived(move |_rcx| {
            //             ColorGradient::new(&[
            //                 Srgba::new(0.0, 0.0, 0.0, 1.0),
            //                 Srgba::new(1.0, 0.0, 0.0, 1.0),
            //             ])
            //         });
            //         let on_change_red = builder.create_callback(
            //             move |new_value: In<f32>, mut world: DeferredWorld| {
            //                 red.set(&mut world, *new_value);
            //             },
            //         );
            //         builder.invoke(
            //             GradientSlider::new()
            //                 .gradient(red_gradient)
            //                 .min(0.)
            //                 .max(255.)
            //                 .value(red)
            //                 // .style(style_slider)
            //                 .precision(1)
            //                 .on_change(on_change_red),
            //         );
            //     });

            // builder.text("SpinBox");
            // builder
            //     .spawn(Node::default())
            //     .styles((style_column, |sb: &mut StyleBuilder| {
            //         sb.align_items(ui::AlignItems::Stretch);
            //     }))
            //     .with_children(|builder| {
            //         let value = builder.create_mutable::<f32>(50.);
            //         let on_change = builder.create_callback(
            //             move |new_value: In<f32>, mut world: DeferredWorld| {
            //                 value.set(&mut world, *new_value);
            //             },
            //         );
            //         builder.invoke(
            //             SpinBox::new()
            //                 .min(0.)
            //                 .max(100.)
            //                 .value(value)
            //                 .on_change(on_change),
            //         );
            //     });

            // builder.text("DisclosureToggle");
            // builder.spawn(Node::default()).with_children(|builder| {
            //     let expanded = builder.create_mutable(false);
            //     let on_change =
            //         builder.create_callback(move |value: In<bool>, mut world: DeferredWorld| {
            //             expanded.set(&mut world, *value);
            //         });
            //     builder.invoke(
            //         DisclosureToggle::new()
            //             .expanded(expanded)
            //             .on_change(on_change),
            //     );
            // });
        });
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}