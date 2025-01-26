mod callback;
mod cond;
mod dyn_children;
mod effect_cell;
mod foreach;
mod insert_when;
mod lcs;
mod memo;
mod mutable;
mod mutate_dyn;
mod mutate_dyn2;
mod owner;
mod signal;
mod style;
mod switch;
mod template;

use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    prelude::IntoSystemConfigs,
};
pub use callback::CreateCallback;
pub use cond::{Cond, CreateCond};
pub use dyn_children::{
    DynChildOf, DynChildSpawner, DynChildSpawnerCommands, DynChildren, Fragment,
};
use effect_cell::update_effects;
pub use foreach::{For, ListItems};
pub use insert_when::InsertWhen;
pub use memo::{CreateMemo, Memo, ReadMemo};
pub use mutable::{CreateMutable, Mutable, ReadMutable, WriteMutable};
pub use mutate_dyn::MutateDyn;
pub use mutate_dyn2::MutateDyn2;
pub use owner::{Owned, OwnedBy};
pub use signal::{IntoSignal, Signal};
pub use style::{StyleDyn, StyleHandle, StyleTuple, Styles};
pub use switch::Switch;
pub use template::{InvokeUiTemplate, UiTemplate};

pub struct ThoriumUiCorePlugin;

impl Plugin for ThoriumUiCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
        app.add_systems(
            PostUpdate,
            (
                dyn_children::mark_children_changed,
                dyn_children::flatten_dyn_children,
            )
                .chain(),
        );
    }
}
