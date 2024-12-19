# TODO

- Dialog animation starts one frame too late; visible glitch.
- Portals and/or fixed positioning
- Scrolling
- ListView
- ColorEdit
- Inspector
- PropertyGrid

# Future

- Node Graph Library
- Overlays
- Migrate Panoply

```rust
commands.spawn(Node::default())
    .children((
        Button::with_label("Close"),
        Node::default().insert(InteractionDisabled),
        Node::default().style(style_button),
        (Node::default(), InteractionDisabled).attach(
            MarkWhen<InteractionDisabled>::(|| disabled)
        ),
    ));
```
