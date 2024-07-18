use bevy::app::App;
use bevy::prelude::Plugin;

mod systems;
pub mod components;

struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
    }
}
