use bevy::{
    ecs::world::DeferredWorld,
    picking::{hover::HoverMap, pointer::PointerId},
    prelude::*,
};
// use thorium_ui_core::Signal;

/// Component which indicates that the entity is interested in knowing when the mouse is hovering
/// over it or any of its children.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Hovering(pub bool);

// Note: previously this was implemented as a Reaction, however it was reacting every frame
// because HoverMap is mutated every frame regardless of whether or not it changed.
pub(crate) fn update_hover_states(
    hover_map: Option<Res<HoverMap>>,
    mut hovers: Query<(Entity, &mut Hovering)>,
    parent_query: Query<&Parent>,
) {
    let Some(hover_map) = hover_map else { return };
    let hover_set = hover_map.get(&PointerId::Mouse);
    for (entity, mut hoverable) in hovers.iter_mut() {
        let is_hovering = match hover_set {
            Some(map) => map.iter().any(|(ha, _)| {
                *ha == entity || parent_query.iter_ancestors(*ha).any(|e| e == entity)
            }),
            None => false,
        };
        if hoverable.0 != is_hovering {
            hoverable.0 = is_hovering;
        }
    }
}

/// Trait which is used to determine if the mouse is hovering over the given entity or a descendant.
pub trait IsHovering {
    /// Returns true if the mouse is hovering over the given entity or a descendant.
    fn is_hovering(&self, entity: Entity) -> bool;
}

impl IsHovering for DeferredWorld<'_> {
    fn is_hovering(&self, entity: Entity) -> bool {
        self.entity(entity)
            .get::<Hovering>()
            .map(|h| h.0)
            .unwrap_or(false)
    }
}
