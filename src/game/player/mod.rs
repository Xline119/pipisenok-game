use bevy::app::Update;
use bevy::prelude::{App, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Startup};
use crate::AppState;
use crate::game::GameState;
use crate::game::player::systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnEnter(AppState::MainMenu), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    bound_player_movement,
                    enemy_hit_player,
                    player_collect_star
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            );
    }
}