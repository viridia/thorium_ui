use bevy::ecs::system::SystemId;
use bevy::ecs::world::DeferredWorld;
use bevy::{color::Srgba, prelude::*, ui};
use thorium_ui_core::{
    Cond, IntoSignal, MutateDyn, Signal, StyleHandle, StyleTuple, Styles, UiTemplate,
};
// use bevy_tabindex::TabIndex;

use crate::materials::SwatchRectMaterial;

use crate::{colors, InheritableFontColor};

fn style_swatch(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Flex;
        node.width = ui::Val::Px(12.);
        node.min_height = ui::Val::Px(12.);
        node.padding = ui::UiRect::all(ui::Val::Px(2.));
    });
    ec.insert(InheritableFontColor(colors::FOREGROUND.into()));
}

fn style_selection(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.border = ui::UiRect::all(ui::Val::Px(1.));
        node.align_self = ui::AlignSelf::Stretch;
        node.justify_self = ui::JustifySelf::Stretch;
        node.flex_grow = 1.;
    });
    ec.insert(Outline {
        color: colors::FOREGROUND.into(),
        width: ui::Val::Px(2.),
        offset: ui::Val::Px(0.),
    });
    ec.insert(BorderColor(colors::U1.into()));
}

/// Color swatch widget. This displays a solid color, and can also display a checkerboard
/// pattern behind the color if it has an alpha of less than 1.
#[derive(Default)]
pub struct Swatch {
    /// Color to display
    pub color: Signal<Srgba>,

    /// For swatch grids, whether this swatch is selected.
    pub selected: Signal<bool>,

    /// Additional styles to be applied to the widget.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_click: Option<SystemId<In<Srgba>>>,
}

impl Swatch {
    /// Create a new swatch.
    pub fn new(color: impl IntoSignal<Srgba>) -> Self {
        Self::default().color(color.into_signal())
    }

    /// Set the color to display.
    pub fn color(mut self, color: impl Into<Signal<Srgba>>) -> Self {
        self.color = color.into();
        self
    }

    /// Set additional styles to be applied to the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when clicked.
    pub fn on_click(mut self, on_click: SystemId<In<Srgba>>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    /// Set whether the swatch should be rendered in a 'selected' state.
    pub fn selected(mut self, selected: impl IntoSignal<bool>) -> Self {
        self.selected = selected.into_signal();
        self
    }
}

impl UiTemplate for Swatch {
    fn build(&self, builder: &mut ChildSpawnerCommands) {
        let color = self.color;
        let selected = self.selected;
        let on_click = self.on_click;

        builder
            .spawn((
                MaterialNode::<SwatchRectMaterial>::default(),
                Name::new("Swatch"),
                Styles((style_swatch, self.style.clone())),
                MutateDyn::new(
                    move |world: DeferredWorld| LinearRgba::from(color.get(&world)),
                    |color, ent| {
                        let material_handle = ent
                            .get::<MaterialNode<SwatchRectMaterial>>()
                            .unwrap()
                            .0
                            .clone();
                        let mut ui_materials = unsafe {
                            ent.world_mut()
                                .get_resource_mut::<Assets<SwatchRectMaterial>>()
                                .unwrap()
                        };
                        if material_handle == Handle::default() {
                            let material = ui_materials.add(SwatchRectMaterial {
                                color: color.to_vec4(),
                                border_radius: Vec4::splat(0.),
                            });
                            ent.insert(MaterialNode(material));
                        } else {
                            ui_materials.get_mut(&material_handle).unwrap().color = color.to_vec4();
                        }
                    },
                ),
            ))
            .observe(
                move |mut trigger: Trigger<Pointer<Click>>,
                      world: DeferredWorld,
                      mut commands: Commands| {
                    trigger.propagate(false);
                    if let Some(on_click) = on_click {
                        let c = color.get(&world);
                        commands.run_system_with(on_click, c);
                    }
                },
            )
            .with_children(|builder| {
                builder.spawn(Cond::new(
                    move |world: DeferredWorld| selected.get(&world),
                    || Spawn((Node::default(), Styles(style_selection))),
                    || (),
                ));
            });
    }
}
