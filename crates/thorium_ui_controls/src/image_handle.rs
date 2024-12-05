#![allow(missing_docs)]

use bevy::{
    ecs::component::{Immutable, StorageType},
    prelude::*,
};
use thorium_ui_headless::handle::HandleOrOwnedPath;

/// A component which allows a UiImage to be specified either by a handle or a path.
/// This is later patched in to the UiImage component.
#[derive(Default, Clone, Debug)]
pub struct UiImageHandle(pub HandleOrOwnedPath<Image>);

impl Component for UiImageHandle {
    type Mutability = Immutable;
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let comp = world.get::<UiImageHandle>(entity).unwrap();
            let handle = match comp.0 {
                HandleOrOwnedPath::Handle(ref handle) => handle.clone(),
                HandleOrOwnedPath::Path(ref path) => {
                    let assets = world.get_resource::<AssetServer>().unwrap();
                    assets.load::<Image>(path)
                }
            };
            if let Some(mut ui_image) = world.get_mut::<ImageNode>(entity) {
                ui_image.image = handle;
            }
        });
    }
}
