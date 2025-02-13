mod calc;
mod callback;
mod computations;
mod cond;
mod effect_cell;
mod foreach;
mod fragment;
mod insert_when;
mod lcs;
mod memo;
mod mutable;
mod owner;
mod signal;
mod style;
mod switch;
mod template;

use bevy::app::{App, Plugin, PostUpdate, Update};
pub use calc::Calc;
pub use callback::CreateCallback;
pub use computations::{ComputationOf, Computations};
pub use cond::Cond;
use effect_cell::update_effects;
pub use foreach::{For, ListItems};
pub use fragment::Fragment;
pub use insert_when::InsertWhen;
pub use memo::{CreateMemo, Memo, ReadMemo};
pub use mutable::{CreateMutable, Mutable, ReadMutable, WriteMutable};
pub use owner::{Owned, OwnedBy};
pub use signal::{IntoSignal, Signal};
pub use style::{StyleHandle, StyleTuple, Styles};
pub use switch::Switch;
pub use template::{Invoke, InvokeWith, SpawnArc, SpawnableListGen, Template, TemplateContext};

pub struct ThoriumUiCorePlugin;

impl Plugin for ThoriumUiCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
        app.add_systems(PostUpdate, fragment::mark_children_changed);
    }
}
