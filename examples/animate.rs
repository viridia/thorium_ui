use bevy::{color::palettes, ecs::world::DeferredWorld, prelude::*, ui};
use thorium_ui::{
    hover::{Hovering, IsHovering},
    tab_navigation::{handle_tab_navigation, TabGroup},
    CreateCallback, CreateMemo, CreateMutable, EntityEffect, InvokeUiTemplate, StyleEntity,
    ThoriumUiCorePlugin,
};
use thorium_ui_controls::{
    animation::{BistableTransition, BistableTransitionState},
    colors, typography, Button, Dialog, DialogBody, DialogFooter, DialogHeader, DisclosureToggle,
    InheritableFontColor, InheritableFontSize, Swatch, ThoriumUiControlsPlugin,
    UseInheritedTextStyles,
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
        node.column_gap = ui::Val::Px(4.);
    });
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ThoriumUiCorePlugin, ThoriumUiControlsPlugin))
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, (change_text_color, close_on_esc))
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
            builder.spawn((Text::new("bistable_transition"), UseInheritedTextStyles));
            let mut row = builder.spawn((
                Node::default(),
                Hovering::default(),
                BistableTransition::new(false, 0.3),
            ));
            let row_id = row.id();
            row.effect(
                move |world: DeferredWorld| world.is_hovering(row_id),
                |hovering, ent| {
                    ent.entry::<BistableTransition>()
                        .and_modify(|mut transition| {
                            transition.set_open(hovering);
                        });
                },
            );
            row.style(style_row).with_children(|builder| {
                let color = builder.create_memo(
                    move |world: DeferredWorld| match world
                        .entity(row_id)
                        .get::<BistableTransition>()
                        .unwrap()
                        .state
                    {
                        BistableTransitionState::Entering => palettes::css::GREEN,
                        BistableTransitionState::Entered => palettes::css::YELLOW,
                        BistableTransitionState::Exiting => palettes::css::ORANGE,
                        BistableTransitionState::Exited => palettes::css::GRAY,
                    },
                    palettes::css::WHITE,
                );
                builder.invoke(Swatch::new(color).style(|ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(|mut node| {
                        node.width = ui::Val::Px(64.);
                    });
                }));
            });

            builder.spawn((Text::new("DisclosureToggle"), UseInheritedTextStyles));
            builder.spawn(Node::default()).with_children(|builder| {
                let expanded = builder.create_mutable(false);
                let on_change = builder.create_callback_arg(
                    move |value: In<bool>, mut world: DeferredWorld| {
                        expanded.set(&mut world, *value);
                    },
                );
                builder.invoke(
                    DisclosureToggle::new()
                        .expanded(expanded)
                        .on_change(on_change),
                );
            });

            builder.spawn((Text::new("Dialog"), UseInheritedTextStyles));
            builder.spawn(Node::default()).with_children(|builder| {
                let open = builder.create_mutable(false);
                let on_open = builder.create_callback(move |mut world: DeferredWorld| {
                    open.set(&mut world, true);
                });
                let on_close = builder.create_callback(move |mut world: DeferredWorld| {
                    open.set(&mut world, false);
                });
                let on_exit = builder.create_callback(move || {
                    println!("Dialog exited");
                });
                builder.invoke(Button::new().labeled("Open").on_click(on_open));
                builder.invoke(
                    Dialog::new()
                        .open(open.signal())
                        .on_close(on_close)
                        .on_exited(on_exit)
                        .children(move |builder| {
                            builder.invoke(DialogHeader::new().children(|builder| {
                                builder.spawn((Text::new("Dialog Header"), UseInheritedTextStyles));
                            }));
                            builder.invoke(DialogBody::new().children(|builder| {
                                builder.spawn((Text::new("Dialog Body"), UseInheritedTextStyles));
                            }));
                            builder.invoke(DialogFooter::new().children(move |builder| {
                                builder.invoke(Button::new().labeled("Close").on_click(on_close));
                            }));
                        }),
                );
            });

            builder.spawn((Text::new("Text"), UseInheritedTextStyles));
            builder
                .spawn((TextLayout::default(), Text::default()))
                .style((typography::text_default, |ec: &mut EntityCommands| {
                    ec.insert(InheritableFontSize(32.));
                    ec.insert(InheritableFontColor(palettes::css::GRAY.into()));
                }))
                .with_children(|builder| {
                    builder.spawn((
                        Text("The quick brown fox jumps over the ".to_string()),
                        TextColor::default(),
                        UseInheritedTextStyles,
                    ));
                    builder.spawn((
                        Text("lazy".to_string()),
                        TextColor::default(),
                        UseInheritedTextStyles,
                        AnimateTextColor { hue: 0. },
                    ));
                    builder.spawn((
                        Text(" dog".to_string()),
                        TextColor::default(),
                        UseInheritedTextStyles,
                    ));
                });
        });
}

#[derive(Component)]
struct AnimateTextColor {
    hue: f32,
}

fn change_text_color(mut q_text: Query<(&mut TextColor, &mut AnimateTextColor)>, time: Res<Time>) {
    for (mut text_style, mut animate) in q_text.iter_mut() {
        animate.hue = (animate.hue + time.delta_secs() * 200.).rem_euclid(360.0);
        text_style.0 = Hsla::new(animate.hue, 1., 0.5, 1.).into();
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
