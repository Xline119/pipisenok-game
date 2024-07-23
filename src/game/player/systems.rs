use bevy::audio::CpalSample;
use bevy::prelude::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::animation::animation::{Animation, AnimationIndices, AnimationTimer, SheetProps};
use crate::game::movement::components::Movement;
use crate::game::player::components::Player;
use crate::game::score::components::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;

const STARTING_TRANSLATION: Vec3 = Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 64.0;

const PLAYER_WIDTH: u32 = 60;
const PLAYER_HEIGHT: u32 = 80;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Movement {
            velocity: 0.0,
            acceleration: 0.0,
            direction: Vec3::ZERO,
        },
        SpriteBundle {
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0)
                .with_scale(Vec3::new(1.7, 1.7, 0.0)),
            texture: asset_server.load("sprites/characters/shinobi/animations-full-002.png"),
            ..default()
        },
        Animation {
            sheet_props: SheetProps {
                cell_size: UVec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
                rows: 3,
                cols: 8
            },
            //animation_direction: AnimationDirection::Still,
            animation_indices: AnimationIndices {
                first: 16,
                last: 23
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating))
        },
        Player {}
    ));
}

pub fn stick_camera_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Movement, &mut AnimationIndices), With<Player>>,
) {
    let (mut movement, mut animation_indices) = query.single_mut();
    movement.direction = Vec3::ZERO;
    animation_indices.first = 16;
    animation_indices.last = 23;

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        movement.direction += Vec3::new(-1.0, 0.0, 0.0);
        movement.acceleration = 1.0;

        animation_indices.first = 8;
        animation_indices.last = 15;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        movement.direction += Vec3::new(1.0, 0.0, 0.0);
        movement.acceleration = 1.0;

        animation_indices.first = 8;
        animation_indices.last = 15;
    }

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        movement.direction += Vec3::new(0.0, 1.0, 0.0);
        movement.acceleration = 1.0;

        animation_indices.first = 8;
        animation_indices.last = 15;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        movement.direction += Vec3::new(0.0, -1.0, 0.0);
        movement.acceleration = 1.0;

        animation_indices.first = 8;
        animation_indices.last = 15;
    }

    if movement.direction.length() > 0.0 {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            movement.acceleration *= 2.0;

            animation_indices.first = 0;
            animation_indices.last = 7;
        }

        movement.direction = movement.direction.normalize();
        movement.velocity = PLAYER_SPEED;
    }
}

pub fn bound_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {

        let x_min = 0.0 + (PLAYER_WIDTH.to_float_sample() / 2.0);
        let x_max = WINDOW_WIDTH - (PLAYER_WIDTH.to_float_sample() / 2.0);
        let y_min = 0.0 + (PLAYER_HEIGHT.to_float_sample() / 2.0);
        let y_max = WINDOW_HEIGHT - (PLAYER_HEIGHT.to_float_sample() / 2.0);

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

pub fn player_collect_star(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Score), With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_transform, mut player_score)) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = (PLAYER_WIDTH * PLAYER_HEIGHT).to_float_sample() / 2.0;
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
