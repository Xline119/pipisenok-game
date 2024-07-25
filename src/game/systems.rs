use bevy::prelude::*;

use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::events::GameOver;
use crate::game::GameState;
use crate::game::player::components::{Lose};

pub fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameState::Paused => {
                next_state.set(GameState::Running);
                info!("Game resumed")
            },
            GameState::Running => {
                next_state.set(GameState::Paused);
                info!("Game paused")
            }
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0),
        ..default()
    });
}
