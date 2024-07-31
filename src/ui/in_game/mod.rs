use bevy::app::App;
use bevy::prelude::{OnEnter, OnExit, Plugin};
use crate::AppState;
use crate::ui::in_game::systems::layout::{despwan_controls_hint, spawn_controls_hint};

pub mod components;
mod systems;

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_controls_hint)
            .add_systems(OnExit(AppState::Game), despwan_controls_hint);
    }
}
