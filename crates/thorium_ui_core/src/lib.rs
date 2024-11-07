mod callback;
mod cond;
mod effect_cell;
mod entity_effect;
mod foreach;
mod lcs;
mod switch;
mod template;

use bevy::app::{App, Plugin, Update};
pub use callback::CreateCallback;
pub use cond::Cond;
use effect_cell::{update_effects, EffectCell};
pub use entity_effect::{EntitEffect, WithEffect};
pub use foreach::ForEach;
pub use switch::Switch;
pub use template::{InvokeUiTemplate, UiTemplate};

pub struct ThoriumUiPlugin;

impl Plugin for ThoriumUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
        app.world_mut()
            .register_component_hooks::<EffectCell>()
            .on_remove(|mut world, entity, _cond| {
                let cell = world.get_mut::<EffectCell>(entity).unwrap();
                let comp = cell.0.clone();
                comp.lock().unwrap().cleanup(&mut world, entity);
            });
    }
}
