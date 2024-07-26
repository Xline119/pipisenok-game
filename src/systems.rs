use bevy::prelude::*;

use crate::AppState;
use crate::game::game::GameState;

pub fn exit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

pub fn play_background_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/avbe-night-adventures.wav"),
        settings: PlaybackSettings::LOOP,
    });
}

pub fn transition_to_game_state(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            next_state.set(AppState::Game);
            info!("Game started")
        }
    }
}

pub fn transition_to_main_menu_state(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_game_state.set(GameState::Paused);
            info!("Returned to Main Menu")
        }
    }
}
