use bevy::{
    color::Srgba,
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    ui,
};
use thorium_ui_core::{
    CreateCallback, For, IntoSignal, ListItems, Signal, StyleHandle, StyleTuple, Styles, Template,
    TemplateContext,
};

use crate::colors;

use super::Swatch;

fn style_swatch_grid(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Grid;
        node.grid_auto_rows = vec![ui::GridTrack::default()];
        node.min_width = ui::Val::Px(16.);
        node.min_height = ui::Val::Px(16.);
        node.row_gap = ui::Val::Px(3.);
        node.column_gap = ui::Val::Px(3.);
    });
}

fn style_swatch(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.min_width = ui::Val::Px(16.);
        node.min_height = ui::Val::Px(16.);
    });
}

fn style_empty_slot(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.min_width = ui::Val::Px(16.);
        node.min_height = ui::Val::Px(16.);
        node.border = ui::UiRect::all(ui::Val::Px(1.));
    });
    ec.insert(BorderColor(colors::U2.lighter(0.01).into()));
}

/// Color swatch widget. This displays a solid color, and can also display a checkerboard
/// pattern behind the color if it has an alpha of less than 1.
pub struct SwatchGrid {
    /// Color to display.
    /// TODO: Should this be `Color` instead? How will we serialize?
    pub colors: Signal<Vec<Srgba>>,

    /// Number of rows and columns
    pub grid_size: UVec2,

    /// The currently selected color.
    pub selected: Signal<Srgba>,

    /// Additional styles to be applied to the grid.
    pub style: StyleHandle,

    /// Callback called when a swatch is clicked
    pub on_change: Option<SystemId<In<Srgba>>>,
}

impl SwatchGrid {
    /// Create a new swatch.
    pub fn new(colors: impl IntoSignal<Vec<Srgba>>) -> Self {
        Self::default().colors(colors.into_signal())
    }

    /// Set the color to display.
    pub fn colors(mut self, colors: impl Into<Signal<Vec<Srgba>>>) -> Self {
        self.colors = colors.into();
        self
    }

    /// Set which color is selected.
    pub fn selected(mut self, selected: impl Into<Signal<Srgba>>) -> Self {
        self.selected = selected.into();
        self
    }

    /// Set the number of rows and columns in the grid.
    pub fn grid_size(mut self, grid_size: UVec2) -> Self {
        self.grid_size = grid_size;
        self
    }

    /// Set additional styles to be applied to the button.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the callback called when clicked.
    pub fn on_change(mut self, on_click: SystemId<In<Srgba>>) -> Self {
        self.on_change = Some(on_click);
        self
    }
}

impl Default for SwatchGrid {
    fn default() -> Self {
        Self {
            colors: Signal::Constant(Vec::new()),
            grid_size: UVec2::new(8, 8),
            selected: Signal::Constant(Srgba::default()),
            style: Default::default(),
            on_change: None,
        }
    }
}

impl Template for SwatchGrid {
    fn build(&self, tc: &mut TemplateContext) {
        let colors = self.colors.clone();
        let num_cells = (self.grid_size.x * self.grid_size.y) as usize;
        let grid_size = self.grid_size;
        let selected = self.selected;
        let on_change = self.on_change;

        let on_click = tc.create_callback_arg(move |color: In<Srgba>, mut commands: Commands| {
            if let Some(on_change) = on_change.as_ref() {
                commands.run_system_with(*on_change, *color)
            }
        });

        tc.spawn((
            Node::default(),
            Name::new("SwatchGrid"),
            Styles((
                style_swatch_grid,
                move |ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(move |mut node| {
                        node.grid_template_columns =
                            vec![ui::RepeatedGridTrack::flex(grid_size.x as u16, 1.)];
                        node.grid_template_rows =
                            vec![ui::RepeatedGridTrack::flex(grid_size.y as u16, 1.)];
                    });
                },
                self.style.clone(),
            )),
            children![For::each(
                move |mut items: InMut<ListItems<Option<(Srgba, bool)>>>, world: DeferredWorld| {
                    let colors = colors.get_clone(&world);
                    let selected_color = selected.get(&world);
                    items.clear();
                    (0..num_cells).for_each(move |i| {
                        if i < colors.len() {
                            let color = colors[i];
                            let is_selected = selected_color == color;
                            items.push(Some((color, is_selected)));
                        } else {
                            items.push(None);
                        }
                    });
                },
                move |color, builder| match color {
                    Some((color, selected)) => {
                        builder.invoke(
                            Swatch::new(*color)
                                .selected(Signal::Constant(*selected))
                                .style(style_swatch)
                                .on_click(on_click),
                        );
                    }
                    None => {
                        builder.spawn((Node::default(), Styles(style_empty_slot)));
                    }
                },
                || (),
            )],
        ));
    }
}
