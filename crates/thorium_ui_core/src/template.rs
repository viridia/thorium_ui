use bevy::ecs::{prelude::*, spawn::SpawnableList};

use crate::{DynChildOf, DynChildSpawner};

/// Old-style template that builds a UI.
pub trait UiTemplate {
    fn build(&self, builder: &mut ChildSpawnerCommands);
}

pub trait InvokeUiTemplate {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self;
}

impl InvokeUiTemplate for ChildSpawnerCommands<'_> {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self {
        template.build(self);
        self
    }
}

/// New-style template that builds child elements for a parent entity.
pub trait Template {
    fn build(&self, builder: &mut DynChildSpawner);
}

/// Wrapper that invokes a template.
pub struct Invoke<B: Template>(pub B);

impl<B: Template> SpawnableList<DynChildOf> for Invoke<B> {
    fn spawn(self, world: &mut World, entity: Entity) {
        world
            .entity_mut(entity)
            .with_related::<DynChildOf>(|builder| {
                self.0.build(builder);
            });
    }

    fn size_hint(&self) -> usize {
        0
    }
}

/// Backwards-compatible invoke that uses the old UiTemplate trait and regular children.
pub struct UiInvoke<B: UiTemplate>(pub B);

impl<B: UiTemplate> SpawnableList<ChildOf> for UiInvoke<B> {
    fn spawn(self, world: &mut World, entity: Entity) {
        world.commands().entity(entity).with_children(|builder| {
            self.0.build(builder);
        });
    }

    fn size_hint(&self) -> usize {
        0
    }
}
