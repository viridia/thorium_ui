use std::sync::Arc;

use accesskit::Role;
use bevy::{a11y::AccessibilityNode, ecs::system::SystemId, prelude::*, ui};
use thorium_ui_core::{
    IntoSignal, Signal, SpawnArc, SpawnableListGen, StyleHandle, StyleTuple, Styles, Template,
    TemplateContext,
};

use crate::{rounded_corners::RoundedCorners, size::Size};

use super::{Button, ButtonVariant};

fn style_tool_palette(ec: &mut EntityCommands) {
    ec.entry::<Node>().and_modify(|mut node| {
        node.display = ui::Display::Grid;
        node.grid_auto_rows = vec![ui::GridTrack::default()];
        node.row_gap = ui::Val::Px(1.);
        node.column_gap = ui::Val::Px(1.);
    });
}

/// ToolPalette - a grid of tool buttons
#[derive(Default)]
pub struct ToolPalette {
    /// The buttons to display.
    pub contents: Option<Arc<dyn SpawnableListGen + Send + Sync>>,

    /// Additional styles to be applied to the palette.
    pub style: StyleHandle,

    /// Number of button columns
    pub columns: u16,
}

impl ToolPalette {
    /// Create a new tool palette.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the children for this element.
    pub fn contents<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }

    /// Set additional styles to be applied to the palette.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }

    /// Set the number of button columns.
    pub fn columns(mut self, columns: u16) -> Self {
        self.columns = columns;
        self
    }
}

impl Template for ToolPalette {
    fn build(&self, builder: &mut TemplateContext) {
        let columns = self.columns;
        // let size = self.size;

        builder.spawn((
            Node::default(),
            Name::new("ToolPalette"),
            AccessibilityNode::from(accesskit::Node::new(Role::Group)),
            Styles((
                style_tool_palette,
                move |ec: &mut EntityCommands| {
                    ec.entry::<Node>().and_modify(move |mut node| {
                        node.grid_template_columns = vec![ui::RepeatedGridTrack::auto(columns)];
                    });
                },
                self.style.clone(),
            )),
            Children::spawn(SpawnArc(self.contents.clone())),
        ));
    }
}

/// A button in a ToolPalette.
pub struct ToolButton {
    /// Color variant - default, primary or danger.
    pub variant: Signal<ButtonVariant>,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: Signal<bool>,

    /// The content to display inside the button.
    pub contents: Option<Arc<dyn SpawnableListGen + Send + Sync>>,

    /// Callback called when clicked
    pub(crate) on_click: Option<SystemId>,

    /// The tab index of the button (default 0).
    pub(crate) tab_index: i32,

    /// Which corners to render rounded.
    pub(crate) corners: RoundedCorners,

    /// If true, set focus to this button when it's added to the UI.
    pub(crate) autofocus: bool,
}

impl ToolButton {
    /// Create a new tool button.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the button color variant.
    pub fn variant(&mut self, variant: impl IntoSignal<ButtonVariant>) -> &mut Self {
        self.variant = variant.into_signal();
        self
    }

    /// Method which switches between `default` and `selected` style variants based on a boolean.
    /// Often used for toggle buttons or toolbar items.
    pub fn selected(mut self, selected: bool) -> Self {
        self.variant = if selected {
            ButtonVariant::Selected
        } else {
            ButtonVariant::Default
        }
        .into_signal();
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set the button disabled state.
    pub fn disabled(&mut self, disabled: impl IntoSignal<bool>) -> &mut Self {
        self.disabled = disabled.into_signal();
        self
    }

    /// Set the child views for this element.
    pub fn contents<L: SpawnableListGen + Send + Sync + 'static>(mut self, elts: L) -> Self {
        self.contents = Some(Arc::new(elts));
        self
    }

    /// Set callback when clicked
    pub fn on_click(mut self, callback: SystemId) -> Self {
        self.on_click = Some(callback);
        self
    }

    /// Set the tab index of the button.
    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }

    /// Set which corners to render rounded.
    pub fn corners(mut self, corners: RoundedCorners) -> Self {
        self.corners = corners;
        self
    }

    /// Set whether to autofocus the button when it's added to the UI.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }
}

impl Default for ToolButton {
    fn default() -> Self {
        Self {
            size: Default::default(),
            variant: Default::default(),
            disabled: Default::default(),
            contents: None,
            on_click: Default::default(),
            tab_index: 0,
            corners: RoundedCorners::None,
            autofocus: false,
        }
    }
}

impl Template for ToolButton {
    fn build(&self, builder: &mut TemplateContext) {
        let mut btn = Button::new()
            .size(self.size)
            .variant(self.variant)
            .disabled(self.disabled)
            .tab_index(self.tab_index)
            .autofocus(self.autofocus)
            .corners(self.corners);
        btn.contents = self.contents.clone();
        btn.on_click = self.on_click;
        Template::build(&btn, builder);
    }
}
