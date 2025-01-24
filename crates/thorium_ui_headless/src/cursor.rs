use bevy::{
    picking::{hover::HoverMap, pointer::PointerId},
    prelude::*,
    winit::cursor::CursorIcon,
};

pub(crate) fn update_cursor(
    mut commands: Commands,
    hover_map: Option<Res<HoverMap>>,
    parent_query: Query<&ChildOf>,
    cursor_query: Query<&CursorIcon>,
    mut q_windows: Query<(Entity, &mut Window, Option<&CursorIcon>)>,
) {
    let cursor = hover_map.and_then(|hover_map| match hover_map.get(&PointerId::Mouse) {
        Some(hover_set) => hover_set.keys().find_map(|entity| {
            cursor_query.get(*entity).ok().or_else(|| {
                parent_query
                    .iter_ancestors(*entity)
                    .find_map(|e| cursor_query.get(e).ok())
            })
        }),
        None => None,
    });

    let mut windows_to_change: Vec<Entity> = Vec::new();
    for (entity, _window, prev_cursor) in q_windows.iter_mut() {
        match (cursor, prev_cursor) {
            (Some(cursor), Some(prev_cursor)) if cursor == prev_cursor => continue,
            (None, None) => continue,
            _ => {
                windows_to_change.push(entity);
            }
        }
    }
    windows_to_change.iter().for_each(|entity| {
        if let Some(cursor) = cursor {
            commands.entity(*entity).insert(cursor.clone());
        } else {
            commands.entity(*entity).insert(CursorIcon::default());
        }
    });
}
