use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update};
use crate::AppState;
use crate::game::GameState;
use crate::game::movement::systems::{update_position};

pub mod components;
mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (update_position)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            );
    }
}
