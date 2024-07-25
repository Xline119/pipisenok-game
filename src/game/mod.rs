use bevy::prelude::*;

use player::PlayerPlugin;
use crate::AppState;
use crate::events::GameOver;
use crate::game::movement::movement::MovementPlugin;
use crate::game::systems::*;

mod systems;
pub mod player;
pub mod location;
pub mod movement;

pub struct GamePlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_event::<GameOver>()
            .add_plugins((PlayerPlugin, MovementPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    toggle_pause,
                )
                    .run_if(in_state(AppState::Game))
            );
    }
}