use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::game::GamePlugin;
use crate::game::location::LocationPlugin;
use crate::systems::*;
use crate::ui::UiPlugin;

pub mod events;
mod systems;
mod game;
mod ui;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        // Bevy
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(
                            Window {
                                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                                resizable: false,
                                ..Default::default()
                            }
                        ),
                        ..Default::default()
                    }
                )
                .set(ImagePlugin::default_nearest())
        )
        // App
        .init_state::<AppState>()
        .add_plugins(UiPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(LocationPlugin)
        // Startup
        .add_systems(Startup, (spawn_camera,
                               //play_background_sound
        ))
        // Update
        .add_systems(
            Update,
            (
                exit_on_escape,
                transition_to_game_state,
                transition_to_main_menu_state
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
