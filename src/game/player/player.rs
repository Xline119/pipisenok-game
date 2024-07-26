use std::cmp::PartialEq;
use std::collections::HashSet;
use std::vec;

use bevy::audio::CpalSample;
use bevy::prelude::{App, AssetServer, ButtonInput, Camera, Commands,
                    Component, default, Entity, EventWriter, in_state, info,
                    IntoSystemConfigs, KeyCode, OnEnter, Plugin, Query, Res, ResMut,
                    Resource, Sprite, SpriteBundle, Time, Timer, TimerMode, Transform,
                    Update, UVec2, Vec3, With, Without};
use bevy::prelude::KeyCode::{
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyS, KeyW, ShiftLeft, ShiftRight,
};
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::utils::HashMap;

use crate::{
    AppState, WINDOW_HEIGHT, WINDOW_WIDTH
};
use crate::animation::animation::{animate, Animation, Animate, AnimationIndices, AnimationTimer, setup_animations, SheetProps, PepaAnimationPlugin};
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
            .add_event::<Animate>()
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
                    bound_player_movement,
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
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 1.0)
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
                (ControlledAction::Run(Direction::Up), vec![ShiftLeft, KeyW, ArrowUp]),
                (ControlledAction::Run(Direction::Left), vec![ShiftLeft, KeyA, ArrowLeft]),
                (ControlledAction::Run(Direction::Right), vec![ShiftLeft, KeyD, ArrowRight]),
                (ControlledAction::Run(Direction::Down), vec![ShiftLeft, KeyS, ArrowDown]),
                (ControlledAction::Run(Direction::UpLeft), vec![ShiftLeft, KeyW, KeyA, ArrowUp, ArrowLeft]),
                (ControlledAction::Run(Direction::UpRight), vec![ShiftLeft, KeyW, KeyD, ArrowUp, ArrowRight]),
                (ControlledAction::Run(Direction::DownLeft), vec![ShiftLeft, KeyS, KeyA, ArrowDown, ArrowLeft]),
                (ControlledAction::Run(Direction::DownRight), vec![ShiftLeft, KeyS, KeyD, ArrowDown, ArrowRight]),
            ]),
            state: ControlledAction::None,
            direction: Direction::Zero
        }
    ));
}

pub fn player_movement(
    mut query: Query<(Entity, &Controls), With<Player>>,
    mut move_event_writer: EventWriter<Move>,
    mut animate_event_writer: EventWriter<Animate>,
) {
    let (player_entity, player_controls) = query.single_mut();

    if player_controls.state.is_move_action() {
        let mut animate_event = Animate {
            entity: player_entity,
            direction: player_controls.state.get_direction(),
            animation_indices: AnimationIndices {
                first: 8,
                last: 15
            }
        };

        info!("Sending animate event: {:?}", &animate_event);
        animate_event_writer.send(animate_event);
    } else if player_controls.state.is_run_action() {
        let mut animate_event = Animate {
            entity: player_entity,
            direction: player_controls.state.get_direction(),
            animation_indices: AnimationIndices {
                first: 0,
                last: 7
            }
        };
        info!("Sending animate event: {:?}", &animate_event);
        animate_event_writer.send(animate_event);
    } else {
        let mut animate_event = Animate {
            entity: player_entity,
            direction: player_controls.state.get_direction(),
            animation_indices: AnimationIndices {
                first: 16,
                last: 23
            }
        };

        info!("Sending animate event: {:?}", &animate_event);
        animate_event_writer.send(animate_event);
    }

    if player_controls.state.is_move_action() {
        let mut move_event = Move {
            entity: player_entity,
            direction: player_controls.state.get_direction(),
            speed: PLAYER_SPEED,
            acceleration: 1.0
        };

        info!("Sending move event: {:?}", &move_event);
        move_event_writer.send(move_event);
    }

    if player_controls.state.is_run_action() || player_controls.state.is_diagonal_run_action() {
        let mut move_event = Move {
            entity: player_entity,
            direction: player_controls.state.get_direction(),
            speed: PLAYER_SPEED,
            acceleration: 2.0
        };

        info!("Sending move event: {:?}", &move_event);
        move_event_writer.send(move_event);
    }
}

pub fn stick_camera_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    time: Res<Time>
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = camera_transform.translation.lerp(player_transform.translation, 2.0 * time.delta_seconds());
    camera_transform.translation = camera_transform.translation.clamp(
        Vec3::new(f32::MIN, -WINDOW_HEIGHT, 0.0),
        Vec3::new(f32::MAX, WINDOW_HEIGHT, 0.0),
    )
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

pub fn bound_player_movement(mut query: Query<&mut Transform, With<Player>>) {
    let mut transform = query.single_mut();

    transform.translation = transform.translation.clamp(
        Vec3::new(f32::MIN, PLAYER_HEIGHT as f32 / 2.0, 0.0),
        Vec3::new(f32::MAX, WINDOW_HEIGHT - (PLAYER_HEIGHT as f32 / 2.0), 0.0)
    );
}
