use bevy::prelude::*;

pub trait UiTemplate {
    fn build(&self, builder: &mut UiBuilder);
}

pub trait InvokeUiTemplate {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self;
}

impl<'w> InvokeUiTemplate for UiBuilder<'w> {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self {
        template.build(self);
        self
    }
}

impl<'w> InvokeUiTemplate for EntityCommands<'w> {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self {
        template.build(&mut UiBuilder(self.reborrow()));
        self
    }
}

impl<'w> InvokeUiTemplate for ChildBuilder<'w> {
    fn invoke<T: UiTemplate>(&mut self, _template: T) -> &mut Self {
        // template.build(&mut UiBuilder(self.reborrow()));
        self
    }
}

pub struct UiBuilder<'w>(EntityCommands<'w>);

// {
//     /// Bevy World
//     world: &'w mut World,

//     /// The entity that will be the parent of all of the children and other resources created
//     /// in this scope.
//     parent: Entity,
// }

impl<'w> UiBuilder<'w> {
    /// Construct a new reactive context.
    // pub fn new(world: &'w mut World, owner: Entity) -> Self {
    //     Self {
    //         world,
    //         parent: owner,
    //     }
    // }

    /// Access to world from reactive context.
    // pub fn world(&self) -> &World {
    //     self.world
    // }

    /// Access to mutable world from reactive context.
    // pub fn world_mut(&mut self) -> &mut World {
    //     self.world
    // }

    /// Returns the parent entity
    pub fn parent(&self) -> Entity {
        self.0.id()
    }

    /// Spawn a new child of the parent entity with the given bundle.
    pub fn spawn(&mut self, bundle: impl Bundle) -> &mut EntityCommands<'w> {
        self.0.with_child(bundle)
    }

    /// Spawn a new, empty child of the parent entity.
    pub fn spawn_empty(&mut self) -> &mut EntityCommands<'w> {
        self.0.with_child(())
    }

    /// Return the commands instance for this builder.
    pub fn commands(&'w mut self) -> Commands<'w, 'w> {
        self.0.commands()
    }

    // / Return an `EntityWorldMut` for the given entity.
    // pub fn entity_mut(&'w mut self, entity: Entity) -> EntityCommands<'_> {
    //     self.0.commands().entity(entity)
    // }

    // /// Create a new callback which is owned by the parent entity.
    // pub fn create_callback<P: Send, M, S: IntoSystem<In<P>, (), M> + 'static>(
    //     &mut self,
    //     callback: S,
    // ) -> Callback<P> {
    //     let id = self.world_mut().register_system(callback);
    //     let result = Callback::new(id);
    //     let parent = self.parent();
    //     match self.world.get_mut::<CallbackOwner>(parent) {
    //         Some(mut owner) => {
    //             owner.add(result);
    //         }
    //         None => {
    //             let mut owner = CallbackOwner::new();
    //             owner.add(result);
    //             self.world.entity_mut(parent).insert(owner);
    //         }
    //     }
    //     result
    // }

    // /// Create a new [`Mutable`] in this context.
    // pub fn create_mutable<T>(&mut self, init: T) -> Mutable<T>
    // where
    //     T: Send + Sync + 'static,
    // {
    //     create_mutable(self.world, self.parent, init)
    // }

    // /// Create a new [`Derived`] in this context. This represents a readable signal which
    // /// is computed from other signals. The result is not memoized, but is recomputed whenever
    // /// the dependencies change.
    // ///
    // /// Arguments:
    // /// * `compute` - The function that computes the output. This will be called with a single
    // ///    parameter, which is an [`Rcx`] object.
    // pub fn create_derived<R: 'static, F: Send + Sync + 'static + Fn(&mut Rcx) -> R>(
    //     &mut self,
    //     compute: F,
    // ) -> Signal<R> {
    //     let derived = create_derived(self.world, compute);
    //     self.world.entity_mut(self.parent).add_child(derived.id());
    //     Signal::Derived(derived)
    // }

    // /// Create a new memoized computation in this context. This represents a readable signal which
    // /// is computed from other signals. The result is memoized, which means that downstream
    // /// dependants will not be notified unless the output changes.
    // ///
    // /// Arguments:
    // /// * `compute` - The function that computes the output. This will be called with a single
    // ///    parameter, which is a [`Rcx`] object.
    // pub fn create_memo<
    //     R: 'static + PartialEq + Send + Sync + Clone,
    //     F: Send + Sync + 'static + Fn(&mut Rcx) -> R,
    // >(
    //     &mut self,
    //     compute: F,
    // ) -> Signal<R> {
    //     let owner = self.parent;
    //     let ticks = self.world_mut().change_tick();
    //     let mut scope = TrackingScope::new(ticks);
    //     let init = compute(&mut Rcx::new(self.world_mut(), owner, &mut scope));
    //     let mutable = self.create_mutable(init);
    //     let signal = mutable.signal();
    //     self.world_mut().entity_mut(mutable.id()).insert((
    //         ReactionCell::new(MemoReaction(compute)),
    //         scope,
    //         Name::new(format!("Memo::<{}>", std::any::type_name::<R>())),
    //     ));

    //     signal
    // }

    // / Create a reactive effect which is owned by the parent entity.
    // pub fn create_effect<F: Send + Sync + 'static + FnMut(&mut Ecx)>(
    //     &mut self,
    //     effect: F,
    // ) -> &mut Self {
    //     let mut scope = TrackingScope::new(self.world().last_change_tick());
    //     let mut reaction = EffectReaction { effect };
    //     let owner = self.parent;
    //     let effect_owner = self.world.spawn_empty().set_parent(owner).id();
    //     reaction.react(effect_owner, self.world, &mut scope);
    //     self.world.entity_mut(effect_owner).insert((
    //         scope,
    //         ReactionCell::new(reaction),
    //         GhostNode::default(),
    //     ));
    //     self
    // }

    // /// Return a reference to the Component `C` on the owner entity of the current
    // /// context, or one of it's ancestors. This searches up the entity tree until it finds
    // /// a component of the given type.
    // pub fn use_inherited_component<C: Component>(&self) -> Option<&C> {
    //     let mut entity = self.parent;
    //     loop {
    //         let ec = self.world.entity(entity).get::<C>();
    //         if ec.is_some() {
    //             return ec;
    //         }
    //         match self.world.entity(entity).get::<Parent>() {
    //             Some(parent) => entity = **parent,
    //             _ => return None,
    //         }
    //     }
    // }

    // /// A general mechanism for dynamically-computed children.
    // ///
    // /// Arguments:
    // /// * compute: A reactive function which computes a result.
    // /// * build: A builder function which consumes the result of the previous argument.
    // ///
    // /// Each time the `compute` function reacts, regardless of whether the result is changed or
    // /// not, the children are despawned and rebuilt.
    // pub fn computed<
    //     D: 'static,
    //     F: Send + Sync + 'static + Fn(&Rcx) -> D,
    //     B: Send + Sync + 'static + Fn(D, &mut UiBuilder),
    // >(
    //     &mut self,
    //     compute: F,
    //     build: B,
    // ) -> &mut Self {
    //     // Create an entity to represent the condition.
    //     let mut owner = self.spawn(Name::new("Computed"));
    //     let owner_id = owner.id();

    //     // Create a tracking scope and reaction.
    //     let mut tracking = TrackingScope::new(owner.world().last_change_tick());
    //     let mut reaction = ComputedReaction { compute, build };

    //     // Safety: this should be safe because we don't use owner any more after this
    //     // point.
    //     let world = unsafe { owner.world_mut() };
    //     // Trigger the initial reaction.
    //     reaction.react(owner_id, world, &mut tracking);
    //     world.entity_mut(owner_id).insert((
    //         GhostNode::default(),
    //         tracking,
    //         ReactionCell::new(reaction),
    //     ));
    //     self
    // }
}

// pub trait CreateChilden {
//     fn create_children(&mut self, spawn_children: impl FnOnce(&mut UiBuilder)) -> &mut Self;
//     fn create_children_mut(&mut self, spawn_children: impl FnMut(&mut UiBuilder)) -> &mut Self;
// }

// impl<'w> CreateChilden for EntityWorldMut<'w> {
//     fn create_children(&mut self, spawn_children: impl FnOnce(&mut UiBuilder)) -> &mut Self {
//         let parent = self.id();
//         self.world_scope(|world| {
//             spawn_children(&mut UiBuilder { world, parent });
//         });
//         self
//     }

//     fn create_children_mut(&mut self, mut spawn_children: impl FnMut(&mut UiBuilder)) -> &mut Self {
//         let parent = self.id();
//         self.world_scope(|world| {
//             spawn_children(&mut UiBuilder { world, parent });
//         });
//         self
//     }
// }

// /// General effect reaction.
// pub struct MemoReaction<
//     R: 'static + PartialEq + Send + Sync + Clone,
//     F: Send + Sync + 'static + Fn(&mut Rcx) -> R,
// >(F);

// impl<
//         R: 'static + PartialEq + Send + Sync + Clone,
//         F: Send + Sync + 'static + Fn(&mut Rcx) -> R,
//     > Reaction for MemoReaction<R, F>
// {
//     fn react(&mut self, owner: Entity, world: &mut World, tracking: &mut TrackingScope) {
//         let mut rcx = Rcx::new(world, owner, tracking);
//         let value = (self.0)(&mut rcx);
//         world.write_mutable(owner, value);
//     }
// }

// /// A reaction that handles the conditional rendering logic.
// struct ComputedReaction<D, F: Fn(&Rcx) -> D, B: Fn(D, &mut UiBuilder)>
// where
//     Self: Send + Sync,
// {
//     compute: F,
//     build: B,
// }

// impl<D, F: Send + Sync + Fn(&Rcx) -> D, B: Send + Sync + Fn(D, &mut UiBuilder)> Reaction
//     for ComputedReaction<D, F, B>
// {
//     fn react(&mut self, owner: Entity, world: &mut World, tracking: &mut TrackingScope) {
//         // Create a reactive context and call the test condition.
//         let re = Rcx::new(world, owner, tracking);
//         let deps: D = (self.compute)(&re);
//         world.entity_mut(owner).despawn_descendants();
//         let mut builder = UiBuilder::new(world, owner);
//         (self.build)(deps, &mut builder);
//     }
// }
