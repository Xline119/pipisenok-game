use bevy::asset::AssetServer;
use bevy::audio::{
    AudioBundle, PlaybackSettings
};
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{Assets, Camera, Commands, Component, default, Deref, DerefMut, Entity, EventWriter, KeyCode, Mut, Query, Res, ResMut, SpriteBundle, TextureAtlasLayout, Time, Timer, TimerMode, Transform, UVec2, Window, With, Without};
use bevy::sprite::TextureAtlas;
use bevy::window::PrimaryWindow;

use crate::events::GameOver;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::movement::components::{Acceleration, Direction, MoveableObjectBundle, Velocity};
use crate::game::player::components::Player;
use crate::game::score::components::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;
use crate::WINDOW_WIDTH;

const STARTING_TRANSLATION: Vec3 = Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 64.0;


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn((
        MoveableObjectBundle {
            velocity: Velocity { value: 0.0 },
            acceleration: Acceleration { value: 0.0 },
            direction: Vec3::ZERO.into(),
            model: SpriteBundle {
                transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0)
                    .with_scale(Vec3::new(1.6, 1.7, 0.0)),
                texture: asset_server.load("sprites/characters/shinobi/Idle.png"),
                ..default()
            }
        },
        TextureAtlas {
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(128), 6, 1, None, None)),
            index: 1,
        },
        AnimationIndices { first: 1, last: 5 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {}
    ));
}

pub fn stick_camera_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Direction, &mut Acceleration), With<Player>>,
) {
    let (mut velocity, mut direction, mut acceleration) = player_query.single_mut();
    direction.value = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.value += Vec3::new(-1.0, 0.0, 0.0);
        acceleration.value = 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.value += Vec3::new(1.0, 0.0, 0.0);
        acceleration.value = 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.value += Vec3::new(0.0, 1.0, 0.0);
        acceleration.value = 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.value += Vec3::new(0.0, -1.0, 0.0);
        acceleration.value = 1.0;
    }

    if direction.value.length() > 0.0 {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            acceleration.value *= 2.0;
        }

        direction.value = direction.value.normalize();
        velocity.value = PLAYER_SPEED;
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
    mut game_over_writer: EventWriter<GameOver>
) {
    if let Ok((player_entity, player_transform, player_score)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                game_over_writer.send(GameOver {
                    score: player_score.value
                });
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
                    settings: PlaybackSettings::DESPAWN,
                });

                commands.entity(star_entity).despawn()
            }
        }
    }
}
