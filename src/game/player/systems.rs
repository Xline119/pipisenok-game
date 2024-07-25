use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::vec;

use bevy::audio::CpalSample;
use bevy::math::CompassOctant;
use bevy::prelude::{AssetServer, ButtonInput, Camera, Commands, Component, default, Entity, EventWriter, info, KeyCode, Query, Res, ResMut, Resource, Sprite, SpriteBundle, Timer, TimerMode, Transform, UVec2, Vec3, With, Without};
use bevy::render::render_resource::encase::private::RuntimeSizedArray;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::animation::animation::{Animation, AnimationIndices, AnimationTimer, SheetProps};
use crate::game::movement::movement::{Direction, Move, Movement};
use crate::game::player::components::Player;

const STARTING_TRANSLATION: Vec3 = Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 64.0;

const PLAYER_WIDTH: u32 = 60;
const PLAYER_HEIGHT: u32 = 80;

#[derive(Component)]
pub struct PlayerSettings {
    controls: HashMap<Direction, Vec<KeyCode>>
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            controls: HashMap::new()
        }
    }
}

impl PlayerSettings {
    pub fn get_direction(&self, pressed_keys: HashSet<KeyCode>) -> Direction {
        if pressed_keys.is_empty() {
            return Direction::Zero;
        }

        for (direction, keys) in self.controls.iter() {
            let is_single_keypress = pressed_keys.len() == 1
                && keys.len() == 2
                && keys.contains(pressed_keys.iter().last().unwrap());
            let is_double_keypress = pressed_keys.len() == 2
                && keys.len() == 4
                && pressed_keys.iter().all(|key| keys.contains(key));

            if is_single_keypress || is_double_keypress {
                return *direction;
            }
        }

        Direction::Zero
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("Spawning Player");

    commands.spawn((
        Movement {
            velocity: 0.0,
            acceleration: 0.0,
            direction: Vec3::ZERO,
        },
        PlayerSettings {
            controls: HashMap::from([
                (Direction::Up, vec![KeyCode::KeyW, KeyCode::ArrowUp]),
                (Direction::Down, vec![KeyCode::KeyS, KeyCode::ArrowDown]),
                (Direction::Right, vec![KeyCode::KeyD, KeyCode::ArrowRight]),
                (Direction::Left, vec![KeyCode::KeyA, KeyCode::ArrowLeft]),
                (Direction::UpRight, vec![KeyCode::KeyW, KeyCode::KeyD, KeyCode::ArrowUp, KeyCode::ArrowRight]),
                (Direction::UpLeft, vec![KeyCode::KeyW, KeyCode::KeyA, KeyCode::ArrowUp, KeyCode::ArrowLeft]),
                (Direction::DownRight, vec![KeyCode::KeyS, KeyCode::KeyD, KeyCode::ArrowDown, KeyCode::ArrowRight]),
                (Direction::DownLeft, vec![KeyCode::KeyS, KeyCode::KeyA, KeyCode::ArrowDown, KeyCode::ArrowLeft])
            ])
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

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite_query: Query<&mut Sprite, With<Player>>,
    mut query: Query<(Entity, &mut AnimationIndices, &PlayerSettings), With<Player>>,
    mut event_writer: EventWriter<Move>
) {
    let (player_entity, mut animation_indices, player_settings) = query.single_mut();
    let mut sprite = sprite_query.single_mut();


    sprite.flip_x = false;
    animation_indices.first = 16;
    animation_indices.last = 23;

    let direction = player_settings.get_direction(keyboard_input.get_pressed().cloned().collect());

    if direction != Direction::Zero {
        let move_event = Move {
            entity: player_entity,
            direction,
            speed: PLAYER_SPEED,
            acceleration: 1.0
        };
        info!("Sending move event: {:?}", &move_event);
        event_writer.send(move_event);
    }
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
