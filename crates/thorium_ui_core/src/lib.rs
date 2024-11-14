mod callback;
mod cond;
mod effect_cell;
mod entity_effect;
mod foreach;
mod insert_when;
mod lcs;
mod memo;
mod mutable;
mod signal;
mod style;
mod switch;
mod template;

use bevy::app::{App, Plugin, Update};
pub use callback::CreateCallback;
pub use cond::Cond;
use effect_cell::update_effects;
pub use entity_effect::{EntitEffect, WithEffect};
pub use foreach::{ForEach, ListItems};
pub use insert_when::InsertWhen;
pub use memo::{CreateMemo, Memo, ReadMemo};
pub use mutable::{CreateMutable, Mutable, ReadMutable, WriteMutable};
pub use signal::{IntoSignal, Signal};
pub use style::{StyleEntity, StyleHandle, StyleTuple};
pub use switch::Switch;
pub use template::{InvokeUiTemplate, UiTemplate};

pub struct ThoriumUiCorePlugin;

impl Plugin for ThoriumUiCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
    }
}
