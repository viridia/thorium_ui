use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct ValueChange<T>(pub T);

impl<T: Send + Sync + 'static> Event for ValueChange<T> {
    type Traversal = &'static ChildOf;

    const AUTO_PROPAGATE: bool = true;
}
