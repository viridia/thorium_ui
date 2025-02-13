//! Example of a simple UI layout

use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, ui};
use thorium_ui::{CreateCallback, Invoke, Styles, Template, ThoriumUiCorePlugin};
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

    commands.spawn((
        Node::default(),
        Styles(style_test),
        UiTargetCamera(camera),
        TabGroup::default(),
        Children::spawn((
            Invoke(ButtonDemo),
            Invoke(IconButtonDemo),
            Invoke(ToolPaletteDemo),
        )),
    ));
}

struct ButtonDemo;

impl Template for ButtonDemo {
    fn build(&self, tc: &mut thorium_ui::TemplateContext) {
        let on_click = tc.create_callback(|| {
            println!("Button clicked!");
        });

        tc.spawn((Text::new("Variants"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn((
                Invoke(
                    Button::new()
                        .contents(|| Spawn((Text::new("Default"), UseInheritedTextStyles)))
                        .on_click(on_click),
                ),
                Invoke(
                    Button::new()
                        .variant(ButtonVariant::Primary)
                        .label("Primary"),
                ),
                Invoke(Button::new().variant(ButtonVariant::Danger).label("Danger")),
                Invoke(
                    Button::new()
                        .variant(ButtonVariant::Selected)
                        .label("Selected"),
                ),
                Invoke(Button::new().minimal(true).label("Minimal")),
            )),
        ));

        tc.spawn((Text::new("Variants (disabled)"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn((
                Invoke(Button::new().label("Default").disabled(true)),
                Invoke(
                    Button::new()
                        .variant(ButtonVariant::Primary)
                        .label("Primary")
                        .disabled(true),
                ),
                Invoke(
                    Button::new()
                        .variant(ButtonVariant::Danger)
                        .label("Danger")
                        .disabled(true),
                ),
                Invoke(
                    Button::new()
                        .variant(ButtonVariant::Selected)
                        .label("Selected")
                        .disabled(true),
                ),
                Invoke(Button::new().minimal(true).label("Minimal").disabled(true)),
            )),
        ));

        tc.spawn((Text::new("Size"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn((
                Invoke(Button::new().label("Size: Xl").size(Size::Xl)),
                Invoke(Button::new().label("Size: Lg").size(Size::Lg)),
                Invoke(Button::new().label("Size: Md").size(Size::Md)),
                Invoke(Button::new().label("Size: Sm").size(Size::Sm)),
                Invoke(Button::new().label("Size: Xs").size(Size::Xs)),
                Invoke(Button::new().label("Size: Xxs").size(Size::Xxs)),
                Invoke(Button::new().label("Size: Xxxs").size(Size::Xxxs)),
            )),
        ));

        tc.spawn((Text::new("Corners"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn((
                Invoke(
                    Button::new()
                        .label("corners: All")
                        .corners(RoundedCorners::All),
                ),
                Invoke(
                    Button::new()
                        .label("corners: Top")
                        .corners(RoundedCorners::Top),
                ),
                Invoke(
                    Button::new()
                        .label("corners: Bottom")
                        .corners(RoundedCorners::Bottom),
                ),
                Invoke(
                    Button::new()
                        .label("corners: Left")
                        .corners(RoundedCorners::Left),
                ),
                Invoke(
                    Button::new()
                        .label("corners: Right")
                        .corners(RoundedCorners::Right),
                ),
                Invoke(
                    Button::new()
                        .label("corners: None")
                        .corners(RoundedCorners::None),
                ),
            )),
        ));
    }
}

struct IconButtonDemo;

impl Template for IconButtonDemo {
    fn build(&self, tc: &mut thorium_ui::TemplateContext) {
        tc.spawn((Text::new("IconButton"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn((
                Invoke(IconButton::new(
                    "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                )),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .minimal(true),
                ),
            )),
        ));

        tc.spawn((Text::new("IconButton Size"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn((
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Xl),
                ),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Lg),
                ),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Md),
                ),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Sm),
                ),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Xs),
                ),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Xxs),
                ),
                Invoke(
                    IconButton::new("embedded://thorium_ui_controls/assets/icons/chevron_left.png")
                        .size(Size::Xxxs),
                ),
            )),
        ));
    }
}

struct ToolPaletteDemo;

impl Template for ToolPaletteDemo {
    fn build(&self, tc: &mut thorium_ui::TemplateContext) {
        tc.spawn((Text::new("ToolPalette"), UseInheritedTextStyles));
        tc.spawn((
            Node::default(),
            Styles(style_row),
            Children::spawn(Invoke(ToolPalette::new().columns(3).contents(|| {
                (
                    Invoke(
                        ToolButton::new()
                            .contents(|| {
                                Invoke(Icon::new(
                                    "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                ))
                            })
                            .selected(true)
                            .corners(RoundedCorners::TopLeft),
                    ),
                    Invoke(ToolButton::new().contents(|| {
                        Invoke(Icon::new(
                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                        ))
                    })),
                    Invoke(
                        ToolButton::new()
                            .contents(|| {
                                Invoke(Icon::new(
                                    "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                ))
                            })
                            .corners(RoundedCorners::TopRight),
                    ),
                    Invoke(
                        ToolButton::new()
                            .contents(|| {
                                Invoke(Icon::new(
                                    "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                ))
                            })
                            .corners(RoundedCorners::BottomLeft),
                    ),
                    Invoke(ToolButton::new().contents(|| {
                        Invoke(Icon::new(
                            "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                        ))
                    })),
                    Invoke(
                        ToolButton::new()
                            .contents(|| {
                                Invoke(Icon::new(
                                    "embedded://thorium_ui_controls/assets/icons/chevron_left.png",
                                ))
                            })
                            .corners(RoundedCorners::BottomRight),
                    ),
                )
            }))),
        ));
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
