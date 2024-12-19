use bevy::prelude::EntityCommands;
use variadics_please::all_tuples_enumerated;

/// `Attachment` - an item, or tuple of items, to attach to an entity, which can be effects,
/// components, or child entities.
pub trait Attachment: Sync + Send {
    /// Method to construct the effect on the target entity.
    fn apply(self, commands: &mut EntityCommands);
}

/// Empty attachment.
impl Attachment for () {
    fn apply(self, _ctx: &mut EntityCommands) {}
}

macro_rules! impl_attach_tuple {
    ($(($idx: tt, $style: ident)),+) => {
        impl<$(
            $style: Attachment + 'static,
        )+> Attachment for ( $( $style, )* ) {
            fn apply(self, builder: &mut EntityCommands) {
                $( self.$idx.apply(builder); )*
            }
        }
    };
}

all_tuples_enumerated!(impl_attach_tuple, 1, 15, E);

pub trait Attach {
    fn attach(&mut self, effect_tuple: impl Attachment) -> &mut Self;
}

impl Attach for EntityCommands<'_> {
    fn attach(&mut self, effect_tuple: impl Attachment) -> &mut Self {
        effect_tuple.apply(self);
        self
    }
}

// /// Attach a child entity
// pub struct Child<A: IntoAttachment>(pub A);

// impl<B: Bundle> Child<B> {
//     pub fn new(bundle: B) -> Self {
//         Self(bundle)
//     }
// }

// impl<B: Bundle> Attachment for Child<B> {
//     fn apply(self, builder: &mut EntityCommands) {
//         builder.with_child(self.0);
//     }
// }
