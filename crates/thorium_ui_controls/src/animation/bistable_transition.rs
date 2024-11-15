use bevy::{ecs::system::SystemId, prelude::*};

/// Plugin that runs the timers for bistable transitions.
pub struct BistableTransitionPlugin;

impl Plugin for BistableTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enter_exit_state_machine);
    }
}

/// Tracks an enter / exit transition. This is useful for widgets like dialog boxes and popup
/// menus which have an opening and closing animation.
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum BistableTransitionState {
    /// Opening animation.
    Entering,

    /// Fully open.
    Entered,

    /// Closing animation.
    Exiting,

    /// Fully closed.
    #[default]
    Exited,
}

impl BistableTransitionState {
    /// Convert the state into a readable string.
    pub fn as_name(&self) -> &str {
        match self {
            BistableTransitionState::Entering => "entering",
            BistableTransitionState::Entered => "entered",
            BistableTransitionState::Exiting => "exiting",
            BistableTransitionState::Exited => "exited",
        }
    }
}

#[derive(Component, Default)]
#[require(TransitionTimer)]
pub struct BistableTransition {
    pub open: bool,
    pub delay: f32,
    pub state: BistableTransitionState,
    pub on_exited: Option<SystemId>,
}

impl BistableTransition {
    /// Construct a new bistable transition.
    pub fn new(open: bool, delay: f32) -> Self {
        Self {
            open,
            delay,
            state: if open {
                BistableTransitionState::Entering
            } else {
                BistableTransitionState::Exited
            },
            on_exited: None,
        }
    }

    pub fn with_exit_callback(mut self, on_exited: SystemId) -> Self {
        self.on_exited = Some(on_exited);
        self
    }

    pub fn set_exit_callback(mut self, on_exited: Option<SystemId>) -> Self {
        self.on_exited = on_exited;
        self
    }

    /// Get the open state of the transition.
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Toggle the open state of the transition.
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}

#[derive(Component, Default)]
pub struct TransitionTimer {
    pub timer: f32,
}

// /// Trait which adds `create_bistable_transition` to [`Cx`].
// pub trait CreateBistableTransition {
//     /// Create a bistable transition: a state machine that toggles between two states, with a delay
//     /// between each transition. This can be used for animated effects such as opening and closing
//     /// a dialog.
//     ///
//     /// # Arguments
//     /// * `open` - A signal which controls the state of the transition. When `open` is `true`, the
//     ///    transition proceed through the `EnterStart`, `Entering`, and `Entered` states. When
//     ///    `open` is `false`, the transition proceeds through the `ExitStart`, `Exiting`, and
//     ///    `Exited` states.
//     /// * `delay` - The duration of the transition, in seconds.
//     fn create_bistable_transition(
//         &mut self,
//         open: Signal<bool>,
//         delay: f32,
//     ) -> Signal<BistableTransitionState>;
// }

// impl<'w> CreateBistableTransition for ChildBuilder<'w> {
//     fn create_bistable_transition(
//         &mut self,
//         open: Signal<bool>,
//         delay: f32,
//     ) -> Signal<BistableTransitionState> {
//         // Create an entity to hold the state machine.
//         let mut entity = self.spawn(GhostNode::default());
//         let entity_id = entity.id();

//         // Effect which updates the state machine when the `open` signal changes.
//         entity.effect(
//             move |world: DeferredWorld| open.get(&world),
//             move |open, ent| {
//                 match ent.get_mut::<BistableTransition>() {
//                     Some(mut ee) => {
//                         ee.open = open;
//                     }
//                     None => {
//                         ent.insert((
//                             BistableTransition {
//                                 open,
//                                 delay,
//                                 ..default()
//                             },
//                             TransitionTimer { ..default() },
//                         ));
//                     }
//                 };
//             },
//         );

//         // Derived signal which returns the current state.
//         self.create_memo(
//             move |world: DeferredWorld| {
//                 world
//                     .entity(entity_id)
//                     .get::<BistableTransition>()
//                     .map(|ee| ee.state)
//                     .unwrap_or(BistableTransitionState::Exited)
//             },
//             BistableTransitionState::Exited,
//         )
//         .into_signal()
//     }
// }

pub fn enter_exit_state_machine(
    mut query: Query<(&mut BistableTransition, &mut TransitionTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut ee, mut tt) in query.iter_mut() {
        match ee.state {
            BistableTransitionState::Entering => {
                if ee.open {
                    tt.timer += time.delta_secs();
                    if tt.timer > ee.delay {
                        ee.state = BistableTransitionState::Entered;
                    }
                } else {
                    ee.state = BistableTransitionState::Exiting;
                    tt.timer = 0.;
                }
            }
            BistableTransitionState::Entered => {
                if !ee.open {
                    ee.state = BistableTransitionState::Exiting;
                    tt.timer = 0.;
                }
            }
            BistableTransitionState::Exiting => {
                if ee.open {
                    ee.state = BistableTransitionState::Entering;
                    tt.timer = 0.;
                } else {
                    tt.timer += time.delta_secs();
                    if tt.timer > ee.delay {
                        ee.state = BistableTransitionState::Exited;
                        if let Some(on_exited) = ee.on_exited {
                            commands.run_system(on_exited);
                        }
                    }
                }
            }
            BistableTransitionState::Exited => {
                if ee.open {
                    ee.state = BistableTransitionState::Entering;
                    tt.timer = 0.;
                }
            }
        }
    }
}
