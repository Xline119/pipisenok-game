use bevy::prelude::*;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::systems::*;

pub mod events;
mod systems;
mod game;
mod main_menu;

fn main() {
    App::new()
        // Bevy
        .add_plugins(DefaultPlugins)
        // App
        .init_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup
        .add_systems(Startup, (spawn_camera, play_background_sound))
        // Update
        .add_systems(
            Update,
            (
                exit_on_escape,
                transition_to_game_state,
                transition_to_main_menu_state
            )
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}
