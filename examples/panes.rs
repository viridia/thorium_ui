//! Example of a simple UI layout

use bevy::{ecs::world::DeferredWorld, input_focus::tab_navigation::TabGroup, prelude::*, ui};
use thorium_ui::{
    CreateCallback, CreateMemo, InvokeUiTemplate, StyleDyn, Styles, ThoriumUiCorePlugin,
};
use thorium_ui_controls::{
    colors, InheritableFontColor, Splitter, SplitterDirection, ThoriumUiControlsPlugin,
    UseInheritedTextStyles,
};

fn style_test(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = Display::Flex;
        node.flex_direction = FlexDirection::Row;
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

fn style_panel(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = Display::Flex;
        node.flex_direction = FlexDirection::Column;
        node.align_items = ui::AlignItems::Stretch;
        node.column_gap = ui::Val::Px(4.);
    });
}

#[derive(Resource)]
pub struct LeftPanelWidth(f32);

#[derive(Resource)]
pub struct RightPanelWidth(f32);

fn main() {
    App::new()
        .insert_resource(LeftPanelWidth(200.0))
        .insert_resource(RightPanelWidth(200.0))
        .add_plugins((DefaultPlugins, ThoriumUiCorePlugin, ThoriumUiControlsPlugin))
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

    commands
        .spawn(Node::default())
        .insert((
            UiTargetCamera(camera),
            TabGroup::default(),
            Styles(style_test),
        ))
        .with_children(|builder| {
            let left_width = builder.create_memo(|res: Res<LeftPanelWidth>| res.0, 0.);
            let on_resize_left =
                builder.create_callback_arg(|value: In<f32>, mut world: DeferredWorld| {
                    world.resource_mut::<LeftPanelWidth>().0 = value.max(100.);
                });
            let right_width = builder.create_memo(|res: Res<RightPanelWidth>| res.0, 0.);
            let on_resize_right =
                builder.create_callback_arg(|value: In<f32>, mut world: DeferredWorld| {
                    world.resource_mut::<RightPanelWidth>().0 = value.max(100.);
                });

            // let dummy_text = "The quick, brown fox jumps over a lazy dog. DJs flock by when MTV ax quiz prog. Junk MTV quiz graced by fox whelps. Bawds jog, flick quartz, vex nymphs. Waltz, bad nymph, for quick jigs vex! Fox nymphs grab quick-jived waltz.";

            builder
                .spawn((
                    Node::default(),
                    Styles(style_panel),
                    StyleDyn::new(
                        |res: Res<LeftPanelWidth>| res.0,
                        |width, ec| {
                            ec.entry::<Node>().and_modify(move |mut node| {
                                node.width = ui::Val::Px(width);
                            });
                        },
                    ),
                ))
                .with_children(|builder| {
                    builder.spawn((Text::new("Left"), UseInheritedTextStyles));
                    // builder.invoke(
                    //     ScrollView::new()
                    //         .style(|ec: &mut EntityCommands| {
                    //             sb.flex_grow(1.);
                    //         })
                    //         .scroll_enable_x(true)
                    //         .scroll_enable_y(true)
                    //         .children(|builder| {
                    //             builder.text(dummy_text.to_owned());
                    //         }),
                    // );
                });

            builder.invoke(Splitter::new().value(left_width).on_change(on_resize_left));

            builder
                .spawn((
                    Node::default(),
                    Styles((style_panel, |ec: &mut EntityCommands| {
                        ec.entry::<Node>().and_modify(|mut node| {
                            node.flex_grow = 1.;
                        });
                    })),
                ))
                .with_children(|builder| {
                    builder.spawn((Text::new("Middle"), UseInheritedTextStyles));
                });

            builder.invoke(
                Splitter::new()
                    .direction(SplitterDirection::VerticalReverse)
                    .value(right_width)
                    .on_change(on_resize_right),
            );

            builder
                .spawn((
                    Node::default(),
                    Styles(style_panel),
                    StyleDyn::new(
                        |res: Res<RightPanelWidth>| res.0,
                        |width, ec| {
                            ec.entry::<Node>().and_modify(move |mut node| {
                                node.width = ui::Val::Px(width);
                            });
                        },
                    ),
                ))
                .with_children(|builder| {
                    builder.spawn((Text::new("Right"), UseInheritedTextStyles));
                });
        });
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
