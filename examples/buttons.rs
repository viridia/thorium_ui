//! Example of a simple UI layout

use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, ui};
use thorium_ui::{CreateCallback, InvokeUiTemplate, StyleEntity, ThoriumUiCorePlugin};
use thorium_ui_controls::{
    colors, rounded_corners::RoundedCorners, size::Size, Button, ButtonVariant, Icon, IconButton,
    InheritableFontColor, ThoriumUiControlsPlugin, ToolButton, ToolPalette, UseInheritedTextStyles,
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
        // .observe(handle_tab_navigation)
        .style(style_test)
        .with_children(|builder| {
            let on_click = builder.create_callback(|| {
                println!("Button clicked!");
            });

            builder.spawn((Text::new("Variants"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder
                        .invoke(
                            Button::new().children(|b| {
                                b.spawn((Text::new("Default"), UseInheritedTextStyles));
                            })
                            .on_click(on_click))

                        .invoke(
                            Button::new()
                                .variant(ButtonVariant::Primary)
                                .labeled("Primary"),
                        )
                        .invoke(
                            Button::new()
                                .variant(ButtonVariant::Danger)
                                .labeled("Danger"),
                        )
                        .invoke(
                            Button::new()
                                .variant(ButtonVariant::Selected)
                                .labeled("Selected"),
                        )
                        .invoke(Button::new().minimal(true).labeled("Minimal"));
                });
            builder.spawn((Text::new("Variants (disabled)"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder
                        .invoke(Button::new().labeled("Default").disabled(true))
                        .invoke(
                            Button::new()
                                .variant(ButtonVariant::Primary)
                                .labeled("Primary")
                                .disabled(true),
                        )
                        .invoke(
                            Button::new()
                                .variant(ButtonVariant::Danger)
                                .labeled("Danger")
                                .disabled(true),
                        )
                        .invoke(
                            Button::new()
                                .variant(ButtonVariant::Selected)
                                .labeled("Selected")
                                .disabled(true),
                        )
                        .invoke(
                            Button::new()
                                .minimal(true)
                                .labeled("Minimal")
                                .disabled(true),
                        );
                });
            builder.spawn((Text::new("Size"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder
                        .invoke(Button::new().labeled("Size: Xl").size(Size::Xl))
                        .invoke(Button::new().labeled("Size: Lg").size(Size::Lg))
                        .invoke(Button::new().labeled("Size: Md").size(Size::Md))
                        .invoke(Button::new().labeled("Size: Sm").size(Size::Sm))
                        .invoke(Button::new().labeled("Size: Xs").size(Size::Xs))
                        .invoke(Button::new().labeled("Size: Xxs").size(Size::Xxs))
                        .invoke(Button::new().labeled("Size: Xxxs").size(Size::Xxxs));
                });
            builder.spawn((Text::new("Corners"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder
                        .invoke(
                            Button::new()
                                .labeled("corners: All")
                                .corners(RoundedCorners::All),
                        )
                        .invoke(
                            Button::new()
                                .labeled("corners: Top")
                                .corners(RoundedCorners::Top),
                        )
                        .invoke(
                            Button::new()
                                .labeled("corners: Bottom")
                                .corners(RoundedCorners::Bottom),
                        )
                        .invoke(
                            Button::new()
                                .labeled("corners: Left")
                                .corners(RoundedCorners::Left),
                        )
                        .invoke(
                            Button::new()
                                .labeled("corners: Right")
                                .corners(RoundedCorners::Right),
                        )
                        .invoke(
                            Button::new()
                                .labeled("corners: None")
                                .corners(RoundedCorners::None),
                        );
                });
            builder.spawn((Text::new("IconButton"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder
                        .invoke(IconButton::new(
                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                        ))
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .minimal(true),
                        );
                });

                builder.spawn((Text::new("IconButton Size"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Xl),
                        )
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Lg),
                        )
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Md),
                        )
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Sm),
                        )
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Xs),
                        )
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Xxs),
                        )
                        .invoke(
                            IconButton::new(
                                "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                            )
                            .size(Size::Xxxs),
                        );
                });

            builder.spawn((Text::new("ToolPalette"), UseInheritedTextStyles));
            builder
                .spawn(Node::default())
                .style(style_row)
                .with_children(|builder| {
                    builder.invoke(ToolPalette::new().columns(3).children(|builder| {
                        builder
                            .invoke(
                                ToolButton::new()
                                    .children(|builder| {
                                        builder.invoke(Icon::new(
                                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                        ));
                                    })
                                    .selected(true)
                                    .corners(RoundedCorners::TopLeft),
                            )
                            .invoke(ToolButton::new().children(|builder| {
                                builder.invoke(Icon::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png"));
                            }))
                            .invoke(
                                ToolButton::new()
                                    .children(|builder| {
                                        builder.invoke(Icon::new(
                                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                        ));
                                    })
                                    .corners(RoundedCorners::TopRight),
                            )
                            .invoke(
                                ToolButton::new()
                                    .children(|builder| {
                                        builder.invoke(Icon::new(
                                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                        ));
                                    })
                                    .corners(RoundedCorners::BottomLeft),
                            )
                            .invoke(ToolButton::new().children(|builder| {
                                builder.invoke(Icon::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png"));
                            }))
                            .invoke(
                                ToolButton::new()
                                    .children(|builder| {
                                        builder.invoke(Icon::new(
                                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                        ));
                                    })
                                    .corners(RoundedCorners::BottomRight),
                            );
                    }));
            });
        });
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
