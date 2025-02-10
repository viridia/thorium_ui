use std::sync::Arc;

use bevy::{
    ecs::bundle::{BundleEffect, DynamicBundle},
    prelude::{Bundle, EntityCommands},
};

/// `StyleTuple` - a variable-length tuple of style functions.
pub trait StyleTuple: Sync + Send {
    /// Method to apply the style to a target entity.
    fn apply(&self, ctx: &mut EntityCommands);

    /// Wrap the tuple in a [`StyleHandle`].
    fn into_handle(self) -> StyleHandle;
}

/// Empty style tuple.
impl StyleTuple for () {
    fn apply(&self, _ctx: &mut EntityCommands) {}

    fn into_handle(self) -> StyleHandle {
        StyleHandle::none()
    }
}

impl<F: Fn(&mut EntityCommands) + Send + Sync + 'static> StyleTuple for F {
    fn apply(&self, ctx: &mut EntityCommands) {
        (self)(ctx);
    }

    fn into_handle(self) -> StyleHandle {
        StyleHandle::new(self)
    }
}

impl StyleTuple for StyleHandle {
    fn apply(&self, ctx: &mut EntityCommands) {
        if let Some(s) = self.style.as_ref() {
            s.apply(ctx);
        }
    }

    fn into_handle(self) -> StyleHandle {
        StyleHandle::new(self)
    }
}

macro_rules! impl_style_tuple {
    ( $($style: ident, $idx: tt);+ ) => {
        impl<$(
            $style: StyleTuple + 'static,
        )+> StyleTuple for ( $( $style, )* ) {
            fn apply(&self, builder: &mut EntityCommands) {
                $( self.$idx.apply(builder); )*
            }

            fn into_handle(self) -> StyleHandle {
                StyleHandle::new(self)
            }
        }
    };
}

impl_style_tuple!(E0, 0);
impl_style_tuple!(E0, 0; E1, 1);
impl_style_tuple!(E0, 0; E1, 1; E2, 2);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14);
impl_style_tuple!(E0, 0; E1, 1; E2, 2; E3, 3; E4, 4; E5, 5; E6, 6; E7, 7; E8, 8; E9, 9; E10, 10; E11, 11; E12, 12; E13, 13; E14, 14; E15, 15);

/// Wrapper type that allows [`StyleTuple`]s to be passed from parent to child views.
#[derive(Default, Clone)]
pub struct StyleHandle {
    /// Reference to the collection of styles.
    pub style: Option<Arc<dyn StyleTuple>>,
}

impl PartialEq for StyleHandle {
    fn eq(&self, other: &Self) -> bool {
        match (&self.style, &other.style) {
            (Some(s1), Some(s2)) => Arc::ptr_eq(s1, s2),
            (None, None) => true,
            _ => false,
        }
    }
}

impl StyleHandle {
    /// Construct a new style handle.
    pub fn new<S: StyleTuple + 'static>(style: S) -> Self {
        Self {
            style: Some(Arc::new(style)),
        }
    }

    /// Construct a placeholder style handle.
    pub fn none() -> Self {
        Self { style: None }
    }
}

pub struct Styles<S: StyleTuple>(pub S);

unsafe impl<S: StyleTuple + 'static> Bundle for Styles<S> {
    fn component_ids(
        _components: &mut bevy::ecs::component::Components,
        _ids: &mut impl FnMut(bevy::ecs::component::ComponentId),
    ) {
    }

    fn get_component_ids(
        _components: &bevy::ecs::component::Components,
        _ids: &mut impl FnMut(Option<bevy::ecs::component::ComponentId>),
    ) {
    }

    fn register_required_components(
        _components: &mut bevy::ecs::component::Components,
        _required_components: &mut bevy::ecs::component::RequiredComponents,
    ) {
    }
}

impl<S: StyleTuple> DynamicBundle for Styles<S> {
    type Effect = Self;

    fn get_components(
        self,
        _func: &mut impl FnMut(bevy::ecs::component::StorageType, bevy::ptr::OwningPtr<'_>),
    ) -> Self::Effect {
        self
    }
}

impl<S: StyleTuple> BundleEffect for Styles<S> {
    fn apply(self, entity: &mut bevy::prelude::EntityWorldMut) {
        let id = entity.id();
        let mut commands = unsafe { entity.world_mut().commands() };
        let mut entity_commands = commands.entity(id);
        self.0.apply(&mut entity_commands);
    }
}
