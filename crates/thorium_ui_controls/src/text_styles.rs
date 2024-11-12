#![allow(missing_docs)]

use bevy::prelude::*;
use thorium_ui_headless::handle::HandleOrOwnedPath;

/// Path to the font asset.
#[derive(Component, Default, Clone, Debug)]
pub struct InheritableFont(pub HandleOrOwnedPath<Font>);

impl InheritableFont {
    /// Create a new `InheritableFont` from a handle.
    pub fn from_handle(handle: Handle<Font>) -> Self {
        Self(HandleOrOwnedPath::Handle(handle))
    }

    /// Create a new `InheritableFont` from a path.
    pub fn from_path(path: &str) -> Self {
        Self(HandleOrOwnedPath::Path(path.to_string()))
    }
}

/// Inherited size of the font.
#[derive(Component, Default, Clone, Debug)]
pub struct InheritableFontSize(pub f32);

/// Inherited text color.
#[derive(Component, Default, Clone, Debug)]
pub struct InheritableFontColor(pub Color);

/// Struct that holds the properties for text rendering, which can be inherited. This allows
/// setting for font face, size and color to be established at a parent level and inherited by
/// child text elements.
///
/// This will be applied to any text nodes that are children of the target entity, unless
/// those nodes explicitly override the properties.
#[derive(Component, Default, Clone, Debug)]
struct ComputedFontStyles {
    /// Path to the font asset.
    pub font: Option<Handle<Font>>,

    /// Inherited size of the font.
    pub font_size: Option<f32>,

    /// Inherited text color.
    pub color: Option<Color>,
}

impl ComputedFontStyles {
    /// True if all text style properties are set.
    pub fn is_final(&self) -> bool {
        self.font.is_some() && self.font_size.is_some() && self.color.is_some()
    }
}

/// A marker component that is used to indicate that the text entity wants to opt-in to using
/// inherited text styles.
#[derive(Component)]
pub struct UseInheritedTextStyles;

pub(crate) fn update_text_styles(
    mut query: Query<(Entity, &mut Text), With<UseInheritedTextStyles>>,
    q_inherited_font: Query<Ref<InheritableFont>, ()>,
    q_inherited_color: Query<Ref<InheritableFontColor>, ()>,
    q_inherited_size: Query<Ref<InheritableFontSize>, ()>,
    parents: Query<&Parent>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    let inherited_changed = q_inherited_font.iter().any(|cmp| cmp.is_changed())
        || q_inherited_color.iter().any(|cmp| cmp.is_changed())
        || q_inherited_size.iter().any(|cmp| cmp.is_changed());
    for (entity, text) in query.iter_mut() {
        if text.is_changed() || inherited_changed {
            commands.entity(entity).insert(compute_inherited_style(
                entity,
                &q_inherited_font,
                &q_inherited_color,
                &q_inherited_size,
                &parents,
                &assets,
            ));
        }
    }
}

pub(crate) fn set_initial_text_style(
    trigger: Trigger<OnAdd, UseInheritedTextStyles>,
    q_inherited_font: Query<Ref<InheritableFont>, ()>,
    q_inherited_color: Query<Ref<InheritableFontColor>, ()>,
    q_inherited_size: Query<Ref<InheritableFontSize>, ()>,
    q_parents: Query<&Parent, ()>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .insert(compute_inherited_style(
            trigger.entity(),
            &q_inherited_font,
            &q_inherited_color,
            &q_inherited_size,
            &q_parents,
            &assets,
        ));
}

fn compute_inherited_style(
    entity: Entity,
    inherited_font: &Query<Ref<InheritableFont>, ()>,
    inherited_color: &Query<Ref<InheritableFontColor>, ()>,
    inherited_size: &Query<Ref<InheritableFontSize>, ()>,
    parents: &Query<&Parent, ()>,
    assets: &AssetServer,
) -> (TextFont, TextColor) {
    let mut styles = ComputedFontStyles::default();
    let mut ancestor = entity;
    loop {
        if styles.font.is_none() {
            if let Ok(font) = inherited_font.get(ancestor) {
                styles.font = match font.0 {
                    HandleOrOwnedPath::Handle(ref h) => Some(h.clone()),
                    HandleOrOwnedPath::Path(ref p) => Some(assets.load::<Font>(p)),
                };
            }
        }
        if styles.color.is_none() {
            if let Ok(color) = inherited_color.get(ancestor) {
                styles.color = Some(color.0);
            }
        }
        if styles.font_size.is_none() {
            if let Ok(size) = inherited_size.get(ancestor) {
                styles.font_size = Some(size.0);
            }
        }
        if styles.is_final() {
            break;
        }
        if let Ok(parent) = parents.get(ancestor) {
            ancestor = parent.get();
        } else {
            break;
        }
    }
    let color = TextColor(styles.color.unwrap_or(Color::WHITE));
    let style = TextFont {
        font: styles.font.unwrap_or_default(),
        font_size: styles.font_size.unwrap_or(12.),
        font_smoothing: default(),
    };
    (style, color)
}
