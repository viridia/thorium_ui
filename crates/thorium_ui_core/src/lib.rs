mod callback;
mod cond;
mod effect_cell;
mod entity_effect;
mod foreach;
mod lcs;
mod memo;
mod switch;
mod template;

use bevy::app::{App, Plugin, Update};
pub use callback::CreateCallback;
pub use cond::Cond;
use effect_cell::update_effects;
pub use entity_effect::{EntitEffect, WithEffect};
pub use foreach::ForEach;
pub use memo::{CreateMemo, Memo, ReadMemo};
pub use switch::Switch;
pub use template::{InvokeUiTemplate, UiTemplate};

pub struct ThoriumUiPlugin;

impl Plugin for ThoriumUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
    }
}
