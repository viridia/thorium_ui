mod attach;
mod callback;
mod cond;
mod effect_cell;
mod effect_hook;
mod foreach;
mod insert_when;
mod lcs;
mod memo;
mod mutable;
mod mutation_effect;
mod signal;
mod style;
mod switch;
mod template;

pub use attach::{Attach, Attachment};
use bevy::app::{App, Plugin, Update};
pub use callback::CreateCallback;
pub use cond::CreateCond;
use effect_cell::update_effects;
pub use effect_hook::CreateHookEffect;
pub use foreach::{CreateForEach, ListItems};
pub use insert_when::InsertWhen;
pub use memo::{CreateMemo, Memo, ReadMemo};
pub use mutable::{CreateMutable, Mutable, ReadMutable, WriteMutable};
pub use mutation_effect::MutateDyn;
pub use signal::{IntoSignal, Signal};
pub use style::{StyleDyn, StyleEntity, StyleHandle, StyleTuple};
pub use switch::CreateSwitch;
pub use template::{InvokeUiTemplate, UiTemplate};

pub struct ThoriumUiCorePlugin;

impl Plugin for ThoriumUiCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
    }
}
