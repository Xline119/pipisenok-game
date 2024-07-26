use std::cmp::PartialEq;
use std::collections::HashSet;
use std::vec;

use bevy::audio::CpalSample;
use bevy::prelude::{
    App, AssetServer, ButtonInput, Camera, Commands, Component,
    default, Entity, EventWriter, in_state, info, IntoSystemConfigs, KeyCode,
    OnEnter, Plugin, Query, Res, ResMut, Resource,
    Sprite, SpriteBundle, Timer, TimerMode, Transform, Update, UVec2, Vec3, With, Without
};
use bevy::prelude::KeyCode::{
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyS, KeyW, ShiftLeft, ShiftRight,
};
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::utils::HashMap;

use crate::{
    AppState, WINDOW_HEIGHT, WINDOW_WIDTH
};
use crate::animation::animation::{
    animate, Animation, AnimationIndices, AnimationTimer, setup_animations, SheetProps
};
use crate::game::controls::controls::{ControlledAction, Controls};
use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, Move, Movement};

const STARTING_TRANSLATION: Vec3 = Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 64.0;

const PLAYER_WIDTH: u32 = 60;
const PLAYER_HEIGHT: u32 = 80;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //TODO: move setup_animations to animation plugin
            .add_systems(
                OnEnter(AppState::Game),
                (
                    spawn_player,
                    setup_animations
                )
                    .chain(),
            )
            .add_systems(OnEnter(AppState::MainMenu), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    stick_camera_to_player,
                    //TODO: move animate to animation plugin
                    animate,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning Player");

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
                cols: 8,
            },
            //animation_direction: AnimationDirection::Still,
            animation_indices: AnimationIndices {
                first: 16,
                last: 23,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
        },
        Player {},
        Controls {
            //TODO: move to resources
            controls_map: HashMap::from([
                (ControlledAction::MoveUp, vec![KeyW, ArrowUp]),
                (ControlledAction::MoveDown, vec![KeyS, ArrowDown]),
                (ControlledAction::MoveRight, vec![KeyD, ArrowRight]),
                (ControlledAction::MoveLeft, vec![KeyA, ArrowLeft]),
                (ControlledAction::MoveUpRight, vec![KeyW, KeyD, ArrowUp, ArrowRight]),
                (ControlledAction::MoveUpLeft, vec![KeyW, KeyA, ArrowUp, ArrowLeft]),
                (ControlledAction::MoveDownRight, vec![KeyS, KeyD, ArrowDown, ArrowRight]),
                (ControlledAction::MoveDownLeft, vec![KeyS, KeyA, ArrowDown, ArrowLeft]),
            ]),
            state: ControlledAction::None,
            combined_state: None
        }
    ));
}

pub fn player_movement(
    mut sprite_query: Query<&mut Sprite, With<Player>>,
    mut query: Query<(Entity, &mut AnimationIndices, &Controls), With<Player>>,
    mut event_writer: EventWriter<Move>,
) {
    let (player_entity, mut animation_indices, player_controls) = query.single_mut();
    let mut sprite = sprite_query.single_mut();

    //TODO: move to animation
    sprite.flip_x = false;
    animation_indices.first = 16;
    animation_indices.last = 23;

    if player_controls.state != ControlledAction::None {
        let mut move_event = Move {
            entity: player_entity,
            direction: player_controls.state.get_direction(),
            speed: PLAYER_SPEED,
            acceleration: 1.0
        };

        //TODO: move to animation
        match player_controls.state.get_direction() {
            Direction::Zero => {}
            Direction::Up => {
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::Down => {
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::Right => {
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::Left => {
                sprite.flip_x = true;
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::UpRight => {
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::UpLeft => {
                sprite.flip_x = true;
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::DownRight => {
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
            Direction::DownLeft => {
                sprite.flip_x = true;
                animation_indices.first = 8;
                animation_indices.last = 15;
            }
        }

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

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

pub fn bound_player_movement(mut player_query: Query<&mut Transform, With<Player>>) {
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
