use bevy::asset::AssetServer;
use bevy::audio::{
    AudioBundle, PlaybackSettings
};
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{
    Commands, default, Entity, EventWriter, KeyCode, Query, Res, SpriteBundle, Time, Transform, Window, With
};
use bevy::window::PrimaryWindow;
use log::info;

use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::events::GameOver;
use crate::game::player::components::{
    Lose, Player
};
use crate::game::score::components::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/characters/shinobi/Idle-cropped.png"),
            ..default()
        },
        Player {},
        Score {
            value: 0
        }
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut speed = PLAYER_SPEED;

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed *= 1.5;
        }

        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * speed * time.delta_seconds()
    }
}

pub fn bound_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0 + (PLAYER_SIZE / 2.0);
        let x_max = window.width() - (PLAYER_SIZE / 2.0);
        let y_min = 0.0 + (PLAYER_SIZE / 2.0);
        let y_max = window.height() - (PLAYER_SIZE / 2.0);

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &Score), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut game_over_writer: EventWriter<GameOver>
) {
    let window = window_query.get_single().unwrap();

    if let Ok((player_entity, player_transform, player_score)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/annihilation-gun-sound.wav"),
                    settings: PlaybackSettings::ONCE,
                });

                game_over_writer.send(GameOver {
                    score: player_score.value
                });
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("images/menu/you_lose.png"),
                        transform: Transform::from_xyz(
                            window.width() / 2.0,
                            window.height() / 2.0,
                            0.0,
                        ),
                        ..default()
                    },
                    Lose {}
                ));
                info!("Score {}", player_score.value);
                commands.entity(player_entity).despawn()
            }
        }
    }
}

pub fn player_collect_star(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Score), With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>
) {
    if let Ok((player_transform, mut player_score)) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius {
                player_score.value += 1;
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/coin_pick.wav"),
                    settings: PlaybackSettings::ONCE,
                });

                commands.entity(star_entity).despawn()
            }
        }
    }
}
