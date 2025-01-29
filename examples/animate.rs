use bevy::{
    color::palettes, ecs::world::DeferredWorld, input_focus::tab_navigation::TabGroup, prelude::*,
    ui,
};
use thorium_ui::{
    hover::{Hovering, IsHovering},
    CreateCallback, CreateMemo, CreateMutable, DynChildren, Invoke, InvokeWith, MutateDyn, Styles,
    Template, ThoriumUiCorePlugin,
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

    commands.spawn((
        Node::default(),
        UiTargetCamera(camera),
        TabGroup::default(),
        Styles(style_test),
        DynChildren::spawn((
            Spawn((Text::new("bistable_transition"), UseInheritedTextStyles)),
            InvokeWith(|builder| {
                let mut row = builder.spawn((
                    Node::default(),
                    Hovering::default(),
                    BistableTransition::new(false, 0.3),
                    Styles(style_row),
                ));
                let row_id = row.id();
                let color = row.create_memo(
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
                row.insert((
                    MutateDyn::new(
                        move |world: DeferredWorld| world.is_hovering(row_id),
                        |hovering, ent| {
                            ent.entry::<BistableTransition>()
                                .and_modify(|mut transition| {
                                    transition.set_open(hovering);
                                });
                        },
                    ),
                    DynChildren::spawn(Invoke(Swatch::new(color).style(
                        |ec: &mut EntityCommands| {
                            ec.entry::<Node>().and_modify(|mut node| {
                                node.width = ui::Val::Px(64.);
                            });
                        },
                    ))),
                ));
            }),
            Spawn((Text::new("DisclosureToggle"), UseInheritedTextStyles)),
            Spawn((
                Node::default(),
                DynChildren::spawn(InvokeWith(|builder| {
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
                })),
            )),
            Spawn((Text::new("Dialog"), UseInheritedTextStyles)),
            Spawn((Node::default(), DynChildren::spawn(Invoke(DialogDemo)))),
            Spawn((Text::new("Text"), UseInheritedTextStyles)),
            Spawn((
                TextLayout::default(),
                Text::default(),
                Styles((typography::text_default, |ec: &mut EntityCommands| {
                    ec.insert(InheritableFontSize(32.));
                    ec.insert(InheritableFontColor(palettes::css::GRAY.into()));
                })),
                DynChildren::spawn((
                    Spawn((
                        Text("The quick brown fox jumps over the ".to_string()),
                        TextColor::default(),
                        UseInheritedTextStyles,
                    )),
                    Spawn((
                        Text("lazy".to_string()),
                        TextColor::default(),
                        UseInheritedTextStyles,
                        AnimateTextColor { hue: 0. },
                    )),
                    Spawn((
                        Text(" dog".to_string()),
                        TextColor::default(),
                        UseInheritedTextStyles,
                    )),
                )),
            )),
        )),
    ));
}

struct DialogDemo;

impl Template for DialogDemo {
    fn build(&self, tc: &mut thorium_ui::TemplateContext) {
        let open = tc.create_mutable(false);
        let on_open = tc.create_callback(move |mut world: DeferredWorld| {
            open.set(&mut world, true);
        });
        let on_close = tc.create_callback(move |mut world: DeferredWorld| {
            open.set(&mut world, false);
        });
        let on_exit = tc.create_callback(move || {
            println!("Dialog exited");
        });
        tc.invoke(Button::new().label("Open").on_click(on_open));
        tc.invoke(
            Dialog::new()
                .open(open.signal())
                .on_close(on_close)
                .on_exited(on_exit)
                .contents(move || {
                    (
                        Invoke(DialogHeader::new().children(|builder| {
                            builder.spawn((Text::new("Dialog Header"), UseInheritedTextStyles));
                        })),
                        Invoke(DialogBody::new().children(|builder| {
                            builder.spawn((Text::new("Dialog Body"), UseInheritedTextStyles));
                        })),
                        Invoke(DialogFooter::new().children(move |_builder| {
                            // builder.invoke(Button::new().label("Close").on_click(on_close));
                        })),
                    )
                }),
        );
    }
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
