use bevy::prelude::*;
use crate::game::GamePlugin;
use crate::systems::*;

pub mod events;
mod systems;
mod game;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_systems(Startup, (spawn_camera, play_background_sound))
        .add_systems(Update, (exit_on_escape, restart_game_on_enter))
        .run();
}
