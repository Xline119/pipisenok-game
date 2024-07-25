use bevy::prelude::*;

use crate::animation::animation::{animate, PepaAnimationPlugin, setup_animations};
use crate::AppState;
use crate::game::GameState;
use crate::game::player::systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(PepaAnimationPlugin)
            //TODO: move setup_animations to animation plugin
            //.init_resource::<PlayerSettings>()
            .add_systems(OnEnter(AppState::Game), (
                spawn_player,
                setup_animations
            ).chain())
            .add_systems(OnEnter(AppState::MainMenu), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    stick_camera_to_player,
                    //TODO: move animate to animation plugin
                    animate
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            );
    }
}