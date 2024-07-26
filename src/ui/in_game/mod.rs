use bevy::app::App;
use bevy::prelude::Plugin;

pub mod components;
mod systems;

struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {}
}
