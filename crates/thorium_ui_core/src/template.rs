use std::sync::Arc;

use bevy::ecs::{prelude::*, spawn::SpawnableList};

use crate::DynChildOf;

/// Template that builds child elements for a parent entity.
pub trait Template {
    fn build(&self, tc: &mut TemplateContext);
    fn size_hint(&self) -> usize {
        // TODO: Since most template have one root, should this default to 1?
        0
    }
}

/// Wrapper that invokes a template.
pub struct Invoke<B>(pub B);

impl<B: Template> SpawnableList<DynChildOf> for Invoke<B> {
    fn spawn(self, world: &mut World, entity: Entity) {
        let mut tc = TemplateContext::new(entity, world);
        self.0.build(&mut tc);
    }

    fn size_hint(&self) -> usize {
        Template::size_hint(&self.0)
    }
}

/// Wrapper that invokes a function with a template context.
pub struct InvokeWith<F: Fn(&mut TemplateContext)>(pub F);

impl<F: Fn(&mut TemplateContext)> SpawnableList<DynChildOf> for InvokeWith<F> {
    fn spawn(self, world: &mut World, entity: Entity) {
        let mut tc = TemplateContext::new(entity, world);
        (self.0)(&mut tc);
    }

    fn size_hint(&self) -> usize {
        0
    }
}

/// Trait that represents a function that can produce a [`SpawnableList`].
pub trait SpawnableListGen: Send + Sync {
    fn spawn(&self, world: &mut World, entity: Entity);
}

impl<S: SpawnableList<DynChildOf>, F: Fn() -> S + Send + Sync> SpawnableListGen for F {
    fn spawn(&self, world: &mut World, entity: Entity) {
        self().spawn(world, entity);
    }
}

/// Wrapper that invokes a function with shared reference to a function that can produce spawns.
pub struct SpawnArc(pub Option<Arc<dyn SpawnableListGen + Send + Sync + 'static>>);

impl SpawnableList<DynChildOf> for SpawnArc {
    fn spawn(self, world: &mut World, entity: Entity) {
        if let Some(indirect) = self.0 {
            indirect.spawn(world, entity);
        }
    }

    fn size_hint(&self) -> usize {
        0
    }
}

/// Builder context for templates. This is similar to `ChildSpawner`, but is different
/// in a number of ways:
/// * It always uses the `DynChildOf` relationship.
/// * It has methods for invoking other templates.
/// * Via trait extension, it has methods for spawning owned items such as mutables and callbacks.
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
    pub fn commands(&mut self) -> Commands {
        self.world.commands()
    }
}
