use bevy::app::App;
use bevy::prelude::Plugin;
use crate::ui::main_menu::MainMenuPlugin;

pub mod main_menu;
pub mod in_game;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MainMenuPlugin);
    }
}
