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

// pub trait BundleBuilder<A: Attachment = ()> {
//     fn spawn(self, builder: &mut EntityCommands);
// }

// impl<B: Bundle, A: Attachment> BundleBuilder<A> for B {
//     fn spawn(self, builder: &mut EntityCommands) {
//         todo!();
//     }
// }

// /// Attach a child entity
// pub struct WithChild<BB: BundleBuilder>(pub BB);

// impl<BB: BundleBuilder> WithChild<BB> {
//     pub fn attach<A: Attachment>(self, attachment: A) -> (Self, A) {
//         todo!();
//     }
// }

// impl<A: Attachment, BB: BundleBuilder<A> + Send + Sync> Attachment for WithChild<BB> {
//     fn apply(self, commands: &mut EntityCommands) {
//         self.0.spawn(commands);
//         // builder.with_children(|b| {
//         //     b.spawn(self.0).attach(self.1);
//         // });
//         // builder.with_child(self.0);
//     }
// }
