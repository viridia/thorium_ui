//! Example which uses states and a switch view.

use bevy::{color::palettes::css, prelude::*, ui};
use thorium_ui::{dyn_children, Switch, ThoriumUiCorePlugin};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Play,
    Pause,
    Intro,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ThoriumUiCorePlugin,
        ))
        .insert_state(GameState::Intro)
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, (close_on_esc, handle_key_input))
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

    commands.spawn((
        Node {
            left: ui::Val::Px(0.),
            top: ui::Val::Px(0.),
            right: ui::Val::Px(0.),
            position_type: ui::PositionType::Absolute,
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            border: ui::UiRect::all(ui::Val::Px(3.)),
            ..default()
        },
        BorderColor(css::ALICE_BLUE.into()),
        UiTargetCamera(camera),
        dyn_children![
            Text::new("Game State: "),
            Switch::new(
                |state: Res<State<GameState>>| *state.get(),
                |cases| {
                    cases
                        .case(GameState::Intro, || Spawn(Text::new("Intro")))
                        .case(GameState::Pause, || Spawn(Text::new("Paused")))
                        .fallback(|| Spawn(Text::new("Playing")));
                },
            ),
        ],
    ));
}

fn handle_key_input(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Intro => next_state.set(GameState::Play),
            GameState::Play => next_state.set(GameState::Pause),
            GameState::Pause => next_state.set(GameState::Play),
        }
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
