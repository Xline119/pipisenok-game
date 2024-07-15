mod systems;
pub mod components;
mod styles;

use bevy::prelude::{App, OnEnter, OnExit, Plugin};
use crate::AppState;
use crate::main_menu::systems::layout::{despwan_main_menu, spawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despwan_main_menu);
    }
}