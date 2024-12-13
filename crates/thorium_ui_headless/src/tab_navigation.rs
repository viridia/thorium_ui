use bevy::{
    a11y::Focus,
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Query, ResMut, SystemParam},
    },
    hierarchy::{Children, Parent},
    input::{ButtonInput, ButtonState},
    input_focus::{FocusKeyboardInput, InputFocus, InputFocusVisible},
    log::*,
    prelude::{Added, KeyCode, Res, Trigger, With, Without},
    ui::Node,
};

/// A component which indicates that an entity wants to participate in tab navigation.
///
/// The rules of tabbing are derived from the HTML specification, and are as follows:
/// * An index >= 0 means that the entity is tabbable via sequential navigation.
///   The order of tabbing is determined by the index, with lower indices being tabbed first.
///   If two entities have the same index, then the order is determined by the order of
///   the entities in the ECS hierarchy (as determined by Parent/Child).
/// * An index < 0 means that the entity is not focusable via sequential navigation, but
///   can still be focused via direct selection.
///
/// Note that you must also add the [`TabGroup`] component to the entity's ancestor in order
/// for this component to have any effect.
#[derive(Debug, Default, Component, Copy, Clone)]
pub struct TabIndex(pub i32);

/// Indicates that this widget should automatically receive focus when it's added.
#[derive(Debug, Default, Component, Copy, Clone)]
pub struct AutoFocus;

/// A component used to mark a tree of entities as containing tabbable elements.
#[derive(Debug, Default, Component, Copy, Clone)]
pub struct TabGroup {
    /// The order of the tab group relative to other tab groups.
    pub order: i32,

    /// Whether this is a 'modal' group. If true, then tabbing within the group (that is,
    /// if the current focus entity is a child of this group) will cycle through the children
    /// of this group. If false, then tabbing within the group will cycle through all non-modal
    /// tab groups.
    pub modal: bool,
}

/// An injectable object that provides tab navigation functionality.
#[doc(hidden)]
#[derive(SystemParam)]
#[allow(clippy::type_complexity)]
pub struct TabNavigation<'w, 's> {
    // Query for tab groups.
    tabgroup: Query<'w, 's, (Entity, &'static TabGroup, &'static Children)>,
    // Query for tab indices.
    tabindex: Query<
        'w,
        's,
        (Entity, Option<&'static TabIndex>, Option<&'static Children>),
        (With<Node>, Without<TabGroup>),
    >,
    // Query for parents.
    parent: Query<'w, 's, &'static Parent, With<Node>>,
}

/// Navigation action for tabbing.
pub enum NavAction {
    /// Navigate to the next focusable entity.
    Next,
    /// Navigate to the previous focusable entity.
    Previous,
    /// Navigate to the first focusable entity.
    First,
    /// Navigate to the last focusable entity.
    Last,
}

impl TabNavigation<'_, '_> {
    /// Navigate to the next focusable entity.
    ///
    /// Arguments:
    /// * `focus`: The current focus entity. If `None`, then the first focusable entity is returned,
    ///    unless `reverse` is true, in which case the last focusable entity is returned.
    /// * `reverse`: Whether to navigate in reverse order.
    pub fn navigate(&self, focus: Option<Entity>, action: NavAction) -> Option<Entity> {
        // If there are no tab groups, then there are no focusable entities.
        if self.tabgroup.is_empty() {
            warn!("No tab groups found");
            return None;
        }

        // Start by identifying which tab group we are in. Mainly what we want to know is if
        // we're in a modal group.
        let mut tabgroup: Option<(Entity, &TabGroup)> = None;
        let mut entity = focus;
        while let Some(ent) = entity {
            if let Ok((tg_entity, tg, _)) = self.tabgroup.get(ent) {
                tabgroup = Some((tg_entity, tg));
                break;
            }
            // Search up
            entity = self.parent.get(ent).ok().map(|parent| parent.get());
        }

        if entity.is_some() && tabgroup.is_none() {
            warn!("No tab group found for focus entity");
            return None;
        }

        self.navigate_in_group(tabgroup, focus, action)
    }

    fn navigate_in_group(
        &self,
        tabgroup: Option<(Entity, &TabGroup)>,
        focus: Option<Entity>,
        action: NavAction,
    ) -> Option<Entity> {
        // List of all focusable entities found.
        let mut focusable: Vec<(Entity, TabIndex)> = Vec::with_capacity(self.tabindex.iter().len());

        match tabgroup {
            Some((tg_entity, tg)) if tg.modal => {
                // We're in a modal tab group, then gather all tab indices in that group.
                if let Ok((_, _, children)) = self.tabgroup.get(tg_entity) {
                    for child in children.iter() {
                        self.gather_focusable(&mut focusable, *child);
                    }
                }
            }
            _ => {
                // Otherwise, gather all tab indices in all non-modal tab groups.
                let mut tab_groups: Vec<(Entity, TabGroup)> = self
                    .tabgroup
                    .iter()
                    .filter(|(_, tg, _)| !tg.modal)
                    .map(|(e, tg, _)| (e, *tg))
                    .collect();
                // Stable sort by group order
                tab_groups.sort_by(compare_tab_groups);

                // Search group descendants
                tab_groups.iter().for_each(|(tg_entity, _)| {
                    self.gather_focusable(&mut focusable, *tg_entity);
                })
            }
        }

        if focusable.is_empty() {
            warn!("No focusable entities found");
            return None;
        }

        // Stable sort by tabindex
        focusable.sort_by(compare_tab_indices);

        let index = focusable.iter().position(|e| Some(e.0) == focus);
        let count = focusable.len();
        let next = match (index, action) {
            (Some(idx), NavAction::Next) => (idx + 1).rem_euclid(count),
            (Some(idx), NavAction::Previous) => (idx + count - 1).rem_euclid(count),
            (None, NavAction::Next) => 0,
            (None, NavAction::Previous) => count - 1,
            (_, NavAction::First) => 0,
            (_, NavAction::Last) => count - 1,
        };
        focusable.get(next).map(|(e, _)| e).copied()
    }

    /// Gather all focusable entities in tree order.
    fn gather_focusable(&self, out: &mut Vec<(Entity, TabIndex)>, parent: Entity) {
        if let Ok((entity, tabindex, children)) = self.tabindex.get(parent) {
            if let Some(tabindex) = tabindex {
                if tabindex.0 >= 0 {
                    out.push((entity, *tabindex));
                }
            }
            if let Some(children) = children {
                for child in children.iter() {
                    // Don't recurse into tab groups
                    if self.tabgroup.get(*child).is_err() {
                        self.gather_focusable(out, *child);
                    }
                }
            }
        } else if let Ok((_, tabgroup, children)) = self.tabgroup.get(parent) {
            if !tabgroup.modal {
                for child in children.iter() {
                    self.gather_focusable(out, *child);
                }
            }
        }
    }
}

fn compare_tab_groups(a: &(Entity, TabGroup), b: &(Entity, TabGroup)) -> std::cmp::Ordering {
    a.1.order.cmp(&b.1.order)
}

// Stable sort which compares by tab index
fn compare_tab_indices(a: &(Entity, TabIndex), b: &(Entity, TabIndex)) -> std::cmp::Ordering {
    a.1 .0.cmp(&b.1 .0)
}

fn handle_auto_focus(
    mut focus: ResMut<InputFocus>,
    mut a11y_focus: ResMut<Focus>,
    query: Query<Entity, (With<TabIndex>, Added<AutoFocus>)>,
) {
    if let Some(entity) = query.iter().next() {
        focus.0 = Some(entity);
        a11y_focus.0 = Some(entity);
    }
}

/// Plugin for handling keyboard input.
pub struct TabNavigationPlugin;

impl Plugin for TabNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_auto_focus);
    }
}

/// Observer function which handles tab navigation.
pub fn handle_tab_navigation(
    mut trigger: Trigger<FocusKeyboardInput>,
    nav: TabNavigation,
    mut focus: ResMut<InputFocus>,
    mut a11y_focus: ResMut<Focus>,
    mut visible: ResMut<InputFocusVisible>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Tab navigation.
    let key_event = &trigger.event().input;
    if key_event.key_code == KeyCode::Tab
        && key_event.state == ButtonState::Pressed
        && !key_event.repeat
    {
        let next = nav.navigate(
            focus.0,
            if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                NavAction::Previous
            } else {
                NavAction::Next
            },
        );
        if next.is_some() {
            trigger.propagate(false);
            focus.0 = next;
            a11y_focus.0 = next;
            visible.0 = true;
        }
    }
}
