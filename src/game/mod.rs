use bevy::prelude::{App, AppExtStates, in_state, IntoSystemConfigs, OnExit, Plugin, States, Update};

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;

use crate::AppState;
use crate::events::GameOver;
use crate::game::movement::MovementPlugin;
use crate::game::systems::{despawn_lose, handle_game_over, restart_game_on_enter, toggle_pause};

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;
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
            .add_plugins((PlayerPlugin, EnemyPlugin, StarPlugin, MovementPlugin))
            .add_systems(Update, restart_game_on_enter.run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), despawn_lose)
            .add_systems(
                Update,
                (
                    toggle_pause,
                    handle_game_over
                )
                    .run_if(in_state(AppState::Game))
            );
    }
}