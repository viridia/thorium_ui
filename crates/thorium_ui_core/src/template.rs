use bevy::ecs::prelude::*;

use crate::DynChildSpawner;

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

pub trait BundleTemplate {
    fn build(&self, builder: &mut DynChildSpawner);
}

#[macro_export]
macro_rules! impl_bundle_template {
    ($t:ty) => {
        impl bevy::ecs::spawn::SpawnableList<thorium_ui_core::DynChildOf> for $t {
            fn spawn(self, world: &mut World, entity: Entity) {
                world
                    .entity_mut(entity)
                    .with_related::<thorium_ui_core::DynChildOf>(|builder| {
                        BundleTemplate::build(&self, builder);
                    });
            }

            fn size_hint(&self) -> usize {
                0
            }
        }
    };
}
