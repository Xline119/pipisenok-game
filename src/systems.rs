use bevy::app::AppExit;
use bevy::audio::{
    AudioBundle, PlaybackSettings
};
use bevy::input::ButtonInput;
use bevy::prelude::{AssetServer, Camera2dBundle, Commands, Entity, Events, info, KeyCode, NextState, Query, Res, ResMut, SpriteBundle, State, Transform, Vec2, With};
use bevy::utils::default;
use bevy::window::{
    PrimaryWindow, Window
};
use rand::random;
use crate::AppState;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::NUMBER_OF_ENEMIES;
use crate::game::GameState;
use crate::game::player::components::{Lose, Player};
use crate::game::score::components::Score;
use crate::game::star::components::Star;
use crate::game::star::NUMBER_OF_STARS;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

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
    mut next_state: ResMut<NextState<AppState>>
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
