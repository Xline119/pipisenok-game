use crate::ui::main_menu::MainMenuPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;
use crate::ui::in_game::InGameUiPlugin;

pub mod in_game;
pub mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, InGameUiPlugin));
    }
}
