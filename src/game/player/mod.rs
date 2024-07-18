use bevy::app::Update;
use bevy::prelude::{App, in_state, IntoSystemConfigs, OnEnter, Plugin};

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
                    stick_camera_to_player,
                    enemy_hit_player,
                    player_collect_star
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            )
            .add_systems(Update, animate_sprite.run_if(in_state(AppState::Game)));
    }
}