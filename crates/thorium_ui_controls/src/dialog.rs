use std::sync::Arc;

use bevy::{
    color::{Alpha, Luminance},
    ecs::{system::SystemId, world::DeferredWorld},
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::{self},
};
use thorium_ui_core::{
    computations, dyn_children, Calc, Cond, DynChildren, Signal, SpawnArc, SpawnableListGen,
    Styles, Template, TemplateContext,
};
use thorium_ui_headless::CoreBarrier;

use crate::{
    animation::{
        AnimatedBackgroundColor, AnimatedScale, AnimatedTransition, BistableTransition,
        BistableTransitionState,
    },
    colors,
    typography::text_default,
    InheritableFontSize,
};

// Dialog background overlay
fn style_dialog_barrier(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.justify_content = ui::JustifyContent::Center;
        node.align_items = ui::AlignItems::Center;
        node.position_type = ui::PositionType::Absolute;
        node.left = ui::Val::Px(0.);
        node.top = ui::Val::Px(0.);
        node.width = ui::Val::Vw(100.);
        node.height = ui::Val::Vh(100.);
    });
    // ec.insert(BorderColor(colors::ANIMATION.into()));
    ec.insert(BackgroundColor(colors::U2.with_alpha(0.0).into()));
    ec.insert(ZIndex(100));
}

fn style_dialog(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Column;
        node.justify_content = ui::JustifyContent::Center;
        node.align_items = ui::AlignItems::Stretch;
        node.position_type = ui::PositionType::Relative;
        node.border = ui::UiRect::all(ui::Val::Px(3.));
        node.width = ui::Val::Px(400.);
    });
    ec.insert(BackgroundColor(colors::U2.into()));
    ec.insert(BorderColor(colors::U1.into()));
    ec.insert(BorderRadius::all(ui::Val::Px(6.0)));
    // .scale(0.5)
    // .transition(&[Transition {
    //     property: TransitionProperty::Transform,
    //     duration: 0.3,
    //     timing: timing::EASE_IN_OUT,
    //     ..default()
    // }])
    // .selector(".entering > &,.entered > &", |ss| ss.scale(1.));
}

const TRANSITION_DURATION: f32 = 0.3;

/// Displays a modal dialog box. This will display the dialog frame and the backdrop overlay.
/// Use the dialog header/body/footer controls to get the standard layout.
pub struct Dialog {
    /// The width of the dialog, one of several standard widths.
    pub width: ui::Val,

    /// Signal that controls whether the dialog is open. Note that when this becomes false,
    /// the dialog will still remain visible until it completes its closing animation.
    pub open: Signal<bool>,

    /// The content to display inside the dialog.
    pub contents: Option<Arc<dyn SpawnableListGen + Send + Sync>>,

    /// Callback called when the dialog's close button is clicked.
    pub on_close: Option<SystemId>,

    /// Callback called when the dialog has completed it's closing animation.
    pub on_exited: Option<SystemId>,
}

impl Default for Dialog {
    fn default() -> Self {
        Self {
            width: ui::Val::Px(400.0),
            open: Signal::Constant(false),
            contents: None,
            on_close: None,
            on_exited: None,
        }
    }
}

impl Dialog {
    /// Creates a new `Dialog`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the width of the dialog.
    pub fn width(mut self, width: ui::Val) -> Self {
        self.width = width;
        self
    }

    /// Sets the signal that controls whether the dialog is open.
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.open = open;
        self
    }

    /// Sets the content of the dialog.
    pub fn contents<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }

    /// Sets the callback called when the dialog's close button is clicked.
    pub fn on_close(mut self, on_close: SystemId) -> Self {
        self.on_close = Some(on_close);
        self
    }

    /// Sets the callback called when the dialog has completed it's closing animation.
    pub fn on_exited(mut self, on_exited: SystemId) -> Self {
        self.on_exited = Some(on_exited);
        self
    }
}

impl Template for Dialog {
    fn build(&self, builder: &mut TemplateContext) {
        let on_close = self.on_close;
        let on_exited = self.on_exited;
        let open = self.open;
        let width = self.width;

        let transition_entity = builder.spawn((
            // GhostNode::default(),
            BistableTransition::new(false, TRANSITION_DURATION).set_exit_callback(on_exited),
            computations![Calc::new(
                move |world: DeferredWorld| open.get(&world),
                |open, ent| {
                    ent.get_mut::<BistableTransition>().unwrap().set_open(open);
                },
            )],
        ));
        let transition_id = transition_entity.id();

        let contents = self.contents.clone();
        builder.spawn(Cond::new(
            move |world: DeferredWorld| {
                world
                    .entity(transition_id)
                    .get::<BistableTransition>()
                    .unwrap()
                    .state
                    != BistableTransitionState::Exited
            },
            move || {
                Spawn((
                    Node::default(),
                    Name::new("Dialog::Overlay"),
                    Styles(style_dialog_barrier),
                    CoreBarrier { on_close },
                    computations![Calc::new(
                        move |world: DeferredWorld| match world
                            .entity(transition_id)
                            .get::<BistableTransition>()
                            .unwrap()
                            .state
                        {
                            BistableTransitionState::Entering
                            | BistableTransitionState::Entered => colors::U2.with_alpha(0.7),
                            BistableTransitionState::Exiting | BistableTransitionState::Exited => {
                                colors::U2.with_alpha(0.0)
                            }
                        },
                        move |color, ent| {
                            AnimatedTransition::<AnimatedBackgroundColor>::start(
                                ent,
                                color,
                                None,
                                TRANSITION_DURATION,
                            );
                        },
                    )],
                    dyn_children!((
                        Node::default(),
                        Name::new("Dialog"),
                        Styles((
                            text_default,
                            style_dialog,
                            move |ec: &mut EntityCommands| {
                                ec.entry::<Node>().and_modify(move |mut node| {
                                    node.width = width;
                                });
                            },
                        )),
                        TabGroup {
                            order: 0,
                            modal: true,
                        },
                        computations![Calc::new(
                            move |world: DeferredWorld| match world
                                .entity(transition_id)
                                .get::<BistableTransition>()
                                .unwrap()
                                .state
                            {
                                BistableTransitionState::Entering => (0.0, 1.0),
                                BistableTransitionState::Exiting => (1.0, 0.0),
                                BistableTransitionState::Entered => (1.0, 1.0),
                                BistableTransitionState::Exited => (0.0, 0.0),
                            },
                            move |(origin, target), ent| {
                                AnimatedTransition::<AnimatedScale>::start(
                                    ent,
                                    Vec3::splat(target),
                                    Some(Vec3::splat(origin)),
                                    TRANSITION_DURATION,
                                );
                            },
                        )],
                        DynChildren::spawn(SpawnArc(contents.clone())),
                    )),
                ))
            },
            || (),
        ));

        // TODO: re-enable this code.
        //                     .observe(|mut trigger: Trigger<Pointer<Pressed>>| {
        //                         // Prevent clicks from propagating to the barrier and closing
        //                         // the dialog.
        //                         trigger.propagate(false);
        //                     })
    }
}

fn style_dialog_header(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.justify_content = ui::JustifyContent::SpaceBetween;
        node.border.bottom = ui::Val::Px(1.);
        node.padding = ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(6.0));
    });
    ec.insert(InheritableFontSize(18.));
    ec.insert(BorderColor(colors::U2.darker(0.01).into()));
}

/// Displays a standard dialog header.
#[derive(Clone, Default)]
pub struct DialogHeader {
    /// The content of the dialog header.
    pub contents: Option<Arc<dyn SpawnableListGen + Send + Sync>>,
}

impl DialogHeader {
    /// Create a new dialog header.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content of the dialog header.
    pub fn contents<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }
}

impl Template for DialogHeader {
    fn build(&self, builder: &mut TemplateContext) {
        builder.spawn((
            Node::default(),
            Styles(style_dialog_header),
            DynChildren::spawn(SpawnArc(self.contents.clone())),
        ));
    }
}

fn style_dialog_body(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Column;
        node.align_items = ui::AlignItems::Stretch;
        node.justify_content = ui::JustifyContent::FlexStart;
        node.padding = ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(6.0));
        node.min_height = ui::Val::Px(200.0);
    });
}

/// Displays a standard dialog body.
#[derive(Clone, Default)]
pub struct DialogBody {
    /// The content of the dialog header.
    pub contents: Option<Arc<dyn SpawnableListGen + Send + Sync>>,
}

impl DialogBody {
    /// Create a new dialog body.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content of the dialog body.
    pub fn contents<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }
}

impl Template for DialogBody {
    fn build(&self, builder: &mut TemplateContext) {
        builder.spawn((
            Node::default(),
            Styles(style_dialog_body),
            DynChildren::spawn(SpawnArc(self.contents.clone())),
        ));
    }
}

fn style_dialog_footer(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.flex_direction = ui::FlexDirection::Row;
        node.justify_content = ui::JustifyContent::FlexEnd;
        node.align_items = ui::AlignItems::Center;
        node.border.top = ui::Val::Px(1.);
        node.padding = ui::UiRect::axes(ui::Val::Px(8.0), ui::Val::Px(6.0));
    });
    ec.insert(BorderColor(colors::U2.darker(0.01).into()));
}

/// Displays a standard dialog footer.
#[derive(Clone, Default)]
pub struct DialogFooter {
    /// The content of the dialog header.
    pub contents: Option<Arc<dyn SpawnableListGen + Send + Sync>>,
}

impl DialogFooter {
    /// Create a new dialog footer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content of the dialog footer.
    pub fn contents<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }
}

impl Template for DialogFooter {
    fn build(&self, builder: &mut TemplateContext) {
        builder.spawn((
            Node::default(),
            Styles(style_dialog_footer),
            DynChildren::spawn(SpawnArc(self.contents.clone())),
        ));
    }
}
