use bevy::{
    ecs::{component::HookContext, system::SystemId, world::DeferredWorld},
    prelude::{
        ChildSpawnerCommands, Component, EntityCommands, EntityWorldMut, In, IntoSystem,
        SystemInput,
    },
};

use crate::{owner::OwnedBy, TemplateContext};

#[derive(Component)]
#[component(on_remove = on_remove_callback_cell::<I>, storage = "SparseSet")]
pub struct CallbackCell<I: SystemInput + Send + Sync>(SystemId<I, ()>);

fn on_remove_callback_cell<I: SystemInput + Send + Sync + 'static>(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let system_id = world
        .entity(context.entity)
        .get::<CallbackCell<I>>()
        .unwrap()
        .0;
    world.commands().unregister_system(system_id);
}

/// Methods for registering scoped one-shot systems.
pub trait CreateCallback {
    /// Registers a scoped one-shot system, with no input, that will be removed when the parent
    /// entity is despawned.
    fn create_callback<M, I: IntoSystem<(), (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<(), ()>;

    /// Registers a scoped one-shot systemm, with input, that will be removed when the
    /// parent entity is despawned.
    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()>;
}

impl CreateCallback for EntityCommands<'_> {
    fn create_callback<M, I: IntoSystem<(), (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<(), ()> {
        let system_id = self.commands().register_system(callback);
        let owner = self.id();
        self.commands()
            .spawn((CallbackCell(system_id), OwnedBy(owner)));
        system_id
    }

    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()> {
        let owner = self.id();
        let system_id = self.commands().register_system(callback);
        self.commands()
            .spawn((CallbackCell(system_id), OwnedBy(owner)));
        system_id
    }
}

impl CreateCallback for ChildSpawnerCommands<'_> {
    fn create_callback<M, I: IntoSystem<(), (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<(), ()> {
        let owner = self.target_entity();
        let system_id = self.commands().register_system(callback);
        self.commands()
            .spawn((CallbackCell(system_id), OwnedBy(owner)));
        system_id
    }

    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()> {
        let owner = self.target_entity();
        let system_id = self.commands().register_system(callback);
        self.commands()
            .spawn((CallbackCell(system_id), OwnedBy(owner)));
        system_id
    }
}

impl CreateCallback for TemplateContext<'_> {
    fn create_callback<M, I: IntoSystem<(), (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<(), ()> {
        let system_id = self.commands().register_system(callback);
        let owner = self.target_entity();
        self.commands()
            .spawn((CallbackCell(system_id), OwnedBy(owner)));
        system_id
    }

    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()> {
        let owner = self.target_entity();
        let system_id = self.commands().register_system(callback);
        self.commands()
            .spawn((CallbackCell(system_id), OwnedBy(owner)));
        system_id
    }
}

impl CreateCallback for EntityWorldMut<'_> {
    fn create_callback<M, I: IntoSystem<(), (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<(), ()> {
        let system_id = unsafe { self.world_mut().register_system(callback) };
        let owner = self.id();
        unsafe {
            self.world_mut()
                .spawn((CallbackCell(system_id), OwnedBy(owner)))
        };
        system_id
    }

    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()> {
        let owner = self.id();
        let system_id = unsafe { self.world_mut().register_system(callback) };
        unsafe {
            self.world_mut()
                .spawn((CallbackCell(system_id), OwnedBy(owner)))
        };
        system_id
    }
}
