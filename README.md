# Thorium UI

Thorium is a library that provides dynamic control-flow and effects for Scenes and UIs built
using the Bevy game engine. The core library provides the following features:

- Conditional children using `Cond` and `Switch`.
- Iterative generation using `For::each()`.
- Dynamic effects.
- Nested templates.
- Scoped registration of one-shot systems, that is, one-shot systems which are tied to an entity.

In addition, `thorium_ui_headless` provides a selection of "headless" (in other words, unstyled)
widget implementations. (Currently work in progress).

## Getting started

To initalize the Thorium system, you'll need to install the `ThoriumUiPlugin` in to your Bevy app.

## Using Thorium

Thorium provides mechanisms for dynamically generating and updating both the children and the
components of an entity. Dynamic constructions operate using a "two-stage" design pattern, where
the first stage extracts some data from the world, and the second stage updates the children
or components of the parent entity based on that data. Take for example the `Cond` component:

```rust
// `builder` is a DynChildBuilder
builder.spawn((
    Node::default(),
    dyn_children![
        Cond::new(
            |counter: Res<Counter>| counter.count & 1 == 0, // Even or odd
            || Spawn(Text::new("Hello")),
            || Spawn(Text::new("Goodbye"))
        )],
));
```

The first argument to `Cond` is a _predicate_ function which returns a boolean result. This
function is registered as a Bevy one-shot system, and can use dependency injection to access
parts of the Bevy world. In the example above, we access a `Counter` resource.

The second argument is the "true" or "then" branch, while the third argument is the "false" or
"else" branch. Each branch returns a `SpawnableList` which can be used to generate one or more
child entities.

Whenever the condition is true, the "true" branch will be called, whereas if the condition is
false, the "false" branch will be called. However, this doesn't happen just once: the predicate
condition is called repeatedly (once every frame), and whenever the condition changes (from true
to false, or from false to true) then the children from the old branch will be despawned, and
the children from the new branch constructed.

The condition will continue to run until the parent entity is despawned.

Conditions can be nested: you can have a condition within a condition.

You may have noticed that `Cond` was spawned as a child of `dyn_children![]`. The `DynChildren`
relationship is similar to the standard Bevy `Children` relationship, except that it understands
how to handle dynamic children, that is, children that may or may not be present based on some
condition. This relationship will be discussed later. For now it is sufficient to note that `Cond`
and other control-flow nodes like `For` and `Switch` can only work within a `DynChildren` list.

**Efficiency considerations**: Because the predicate function is called every frame, you should
probably avoid doing any really expensive calculations within it.

**Maintaining Correctness**: When the condition changes, the entities in the old branch are despawned
using `despawn`. This will remove any child nodes that were created from the previous
branch, in effect undoing the effects of that branch. However, the framework cannot undo other
kinds of actions like issuing `Commands` which are possible using the `SpawnWith` interface.
So it is important to only call methods that spawn child entities, or are otherwise safe.

> [!NOTE]
> The name `Cond` is short for `conditional` and comes from LISP. We can't use `if` because that's
> a reserved word in Rust.

### `Switch`

The `Switch` component acts like a switch statement in C. This can be particularly useful in
conjunction with Bevy game states:

```rust
Switch::new(
    |state: Res<State<GameState>>| state.get().clone(),
    |cases| {
        cases
            .case(GameState::Intro, || Spawn(Text::new("Intro")))
            .case(GameState::Pause, || Spawn(Text::new("Paused")))
            .fallback(|| Spawn(Text::new("Playing")));
    },
),
```

The first argument is a function which returns a value. The value must be of a type that implements
`PartialEq`.

Each `.case()` takes two arguments: a match value, and a builder function. The builder function
is called whenever the case value matches the switch value.

The `fallback` case is invoked if none of the other cases match. It's equivalent to the `default`
keyword in C.

Like `Cond`, this sets up a node which runs the first argument continuously. The switch cases
are called whenever the output changes.

### `For::each()` and `For::each_cmp()`

The `For::each()` method takes an array, and creates child nodes for each array element. When
the array changes, it does a "diff" of the old array elements with the new ones. This diff is
then used to generate or despawn the children representing the array elements that changed; the
other children are not affected.

```rust
For::each(
    |list: Res<List>| list.items.clone().into_iter(),
    move |name, builder| {
        builder.spawn(Text::new(name));
    },
    |builder| {
        builder.spawn(Text::new("No items"));
    },
)
```

The first argument is a one-shot system that returns an iterator.

The second argument is called for each array element. It takes the value of the element and
a builder.

`.for_each()` also takes a third, "fallback" parameter, which is used when the array is empty,
so you can print messages like "no results found". If you don't need this feature, just give it
an empty closure.

In order to do the diff operation, `.for_each()` requires that the array elements be of a type
that implements `PartialEq`. However, if you want to use other kinds of data (or want to compare
the items differently), you can use the variant method `.for_each_cmp()` which accepts a custom
comparator function.

### Dynamic Effects

A dynamic effect is a small satellite entity which is attached to the primary entity by an
owner relationship. Dynamic effects continually poll for changes to their inputs, and if a change
occurs, they modify the primary entity in some way.

Here's an effect which modifies the border color.

```rust
entity.insert(
    MutateDyn::new(
        |counter: Res<Counter>| counter.count & 1 == 0,
        |even, entity| {
            entity.entry::<BorderColor>().and_modify(|mut border| {
                entity.insert(BorderColor(if even {
                    css::MAROON.into()
                } else {
                    css::LIME.into()
                }));
            });
        },
    )
)
```

The `effects` method takes either a single effect, or a tuple of effects. Examples of effects
are:

- `InsertWhen` - insert or remove a component based on a boolean condition.
- `Calc` - perform general mutations on an entity

`Calc::new()` takes two arguments: the first argument is a one-shot system that returns a
value. The second argument is called once at the next sync point, and is called again whenever the
value changes. The arguments to the second function are the value, and an `EntityWorldMut` instance.

**Maintaining Correctness**: Having an `EntityWorldMut` means that you can do pretty much anything
you want to the entity. However, unlike the other Thorium methods, `Calc` does not do any
kind of automatic cleanup - it doesn't know how to undo the previous changes. So you will need to
ensure that whatever changes you make to the entity completely overwrite whatever you did the
last time.

In the example above, we change the border color of the entity, which effectively overwrites
the border color set by the previous call.

## Implementation

Thorium relies heavily on the "ghost nodes" feature introduced in Bevy 0.15. Ghost nodes allow
for entities which are hidden, but whose children are rendered in their place.

For example, when you call `.cond()`, it creates a ghost node representing the condition; the actual
branch entities are children of that condition node. When the condition changes, it completely
clears the children of the conditional node. Only the children of the condition node are affected,
other siblings of the condition node are not touched. Without ghost nodes, we would have to track
exactly which child nodes were created by the branch, so that we could erase them later.
