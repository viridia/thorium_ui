use bevy::{
    ecs::{component::ComponentId, system::SystemId, world::DeferredWorld},
    prelude::{
        require, BuildChildren, ChildBuild, ChildBuilder, Component, Entity, EntityCommands, In,
        IntoSystem, SystemInput,
    },
    ui::experimental::GhostNode,
};

#[derive(Component)]
#[component(on_remove = on_remove_callback_cell::<I>, storage = "SparseSet")]
#[require(GhostNode)]
pub struct CallbackCell<I: SystemInput + Send + Sync>(SystemId<I, ()>);

fn on_remove_callback_cell<I: SystemInput + Send + Sync + 'static>(
    mut world: DeferredWorld,
    entity: Entity,
    _: ComponentId,
) {
    let system_id = world.entity(entity).get::<CallbackCell<I>>().unwrap().0;
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
        self.with_child(CallbackCell(system_id));
        system_id
    }

    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()> {
        let system_id = self.commands().register_system(callback);
        self.with_child(CallbackCell(system_id));
        system_id
    }
}

impl CreateCallback for ChildBuilder<'_> {
    fn create_callback<M, I: IntoSystem<(), (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<(), ()> {
        let mut entity = self.spawn_empty();
        let system_id = entity.commands().register_system(callback);
        entity.insert(CallbackCell(system_id));
        system_id
    }

    fn create_callback_arg<M, A: Send + Sync + 'static, I: IntoSystem<In<A>, (), M> + 'static>(
        &mut self,
        callback: I,
    ) -> SystemId<In<A>, ()> {
        let mut entity = self.spawn_empty();
        let system_id = entity.commands().register_system(callback);
        entity.insert(CallbackCell(system_id));
        system_id
    }
}
