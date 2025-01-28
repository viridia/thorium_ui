use bevy::ecs::{prelude::*, spawn::SpawnableList};

use crate::DynChildOf;

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
    fn build(&self, tc: &mut TemplateContext);
    fn size_hint(&self) -> usize {
        // TODO: Since most template have one root, should this default to 1?
        0
    }
}

/// Wrapper that invokes a template.
pub struct Invoke<B: Template>(pub B);

impl<B: Template> SpawnableList<DynChildOf> for Invoke<B> {
    fn spawn(self, world: &mut World, entity: Entity) {
        let mut tc = TemplateContext::new(entity, world);
        self.0.build(&mut tc);
    }

    fn size_hint(&self) -> usize {
        Template::size_hint(&self.0)
    }
}

pub struct TemplateContext<'w> {
    target: Entity,
    world: &'w mut World,
}

impl<'w> TemplateContext<'w> {
    pub fn new(target: Entity, world: &'w mut World) -> Self {
        Self { target, world }
    }

    pub fn target_entity(&self) -> Entity {
        self.target
    }

    /// Spawns an entity with the given `bundle` and an `R` relationship targeting the `target`
    /// entity this spawner was initialized with.
    pub fn spawn(&mut self, bundle: impl Bundle) -> EntityWorldMut<'_> {
        self.world.spawn((DynChildOf(self.target), bundle))
    }

    /// Spawns an entity with an `R` relationship targeting the `target`
    /// entity this spawner was initialized with.
    pub fn spawn_empty(&mut self) -> EntityWorldMut<'_> {
        self.world.spawn(DynChildOf(self.target))
    }

    /// Invoke a template on the target entity. Any children spawned by the template will be
    /// children of the target entity.
    pub fn invoke(&mut self, template: impl Template) -> &mut Self {
        let mut tc = TemplateContext::new(self.target, self.world);
        template.build(&mut tc);
        self
    }

    /// Creates a new [`Commands`] instance that writes to the world's command queue
    /// Use [`World::flush`] to apply all queued commands
    /// TODO: We can get rid this, all we really need is register_system
    pub fn commands(&mut self) -> Commands {
        self.world.commands()
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
