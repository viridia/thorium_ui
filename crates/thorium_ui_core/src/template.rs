use bevy::ecs::prelude::*;

pub trait UiTemplate {
    fn build(&self, builder: &mut ChildSpawnerCommands);
}

pub trait InvokeUiTemplate {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self;
}

impl InvokeUiTemplate for ChildSpawnerCommands<'_> {
    fn invoke<T: UiTemplate>(&mut self, template: T) -> &mut Self {
        template.build(self);
        self
    }
}
