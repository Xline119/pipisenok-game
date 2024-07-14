use bevy::app::AppExit;
use bevy::audio::{
    AudioBundle, PlaybackSettings
};
use bevy::input::ButtonInput;
use bevy::prelude::{
    AssetServer, Camera2dBundle, Commands, Entity, Events, KeyCode, Query, Res, ResMut, SpriteBundle, Transform, Vec2, With
};
use bevy::utils::default;
use bevy::window::{
    PrimaryWindow, Window
};
use rand::random;

use crate::game::enemy::components::Enemy;
use crate::game::enemy::NUMBER_OF_ENEMIES;
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

pub fn restart_game_on_enter(
    mut commands: Commands,
    mut lose_query: Query<Entity, With<Lose>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    stars_query: Query<Entity, With<Star>>,
    enemy_query: Query<Entity, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    if keyboard_input.just_pressed(KeyCode::Enter) {
        if let Ok(lose_entity) = lose_query.get_single_mut() {
            commands.entity(lose_entity).despawn();
            for stars_entity in stars_query.iter() {
                commands.entity(stars_entity).despawn();
            }
            for enemy_entity in enemy_query.iter() {
                commands.entity(enemy_entity).despawn()
            }

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        window.width() / 2.0,
                        window.height() / 2.0,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/characters/shinobi/Idle-cropped.png"),
                    ..default()
                },
                Player {},
                Score {
                    value: 0
                }
            ));

            for _ in 0..NUMBER_OF_ENEMIES {
                let random_x = random::<f32>() * window.width();
                let random_y = random::<f32>() * window.height();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(random_x, random_y, 0.0),
                        texture: asset_server.load("sprites/balls/ball_red_large.png"),
                        ..default()
                    },
                    Enemy {
                        direction: Vec2::new(random(), random()).normalize(),
                    },
                ));
            }

            for _ in 0..NUMBER_OF_STARS {
                let random_x = random::<f32>() * window.width();
                let random_y = random::<f32>() * window.height();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(random_x, random_y, 0.0),
                        texture: asset_server.load("sprites/balls/star.png"),
                        ..default()
                    },
                    Star {},
                ));
            }
        }
    }
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
