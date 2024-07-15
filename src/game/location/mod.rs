use bevy::app::{App, Plugin};
use bevy::prelude::OnEnter;

use crate::AppState;
use crate::game::location::systems::{despawn_forest_location, spawn_forest_location};

pub mod components;
pub mod systems;

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_forest_location)
            .add_systems(OnEnter(AppState::MainMenu), despawn_forest_location);
    }
}