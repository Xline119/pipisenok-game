use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::game::game::GamePlugin;
use crate::game::location::location::LocationPlugin;
use crate::logo::logo::LogoPlugin;
use crate::systems::*;
use crate::ui::UiPlugin;

pub mod animation;
pub mod game;
pub mod logo;
pub mod systems;
pub mod ui;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(get_window_settings())
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((UiPlugin, GamePlugin, LocationPlugin, LogoPlugin))
        .init_state::<AppState>()
        .add_systems(
            Update,
            (
                exit_on_escape,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    Logo,
    #[default]
    Loading,
    MainMenu,
    Game,
    GameOver,
}

fn get_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            title: "Last of Pipisenok".to_string(),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}
