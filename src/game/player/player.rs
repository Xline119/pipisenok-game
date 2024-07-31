use crate::animation::animation::{
    animate_v2, listen_for_texture_change, AnimateEvent, Animation, AnimationAsset,
    AnimationAssets, AnimationIndices, PepaAnimationPlugin,
};
use crate::game::controls::controls::{Actions, ControlledAction, Controls};
use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, MoveEvent, Movement};
use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::asset::ErasedAssetLoader;
use bevy::audio::CpalSample;
use bevy::prelude::KeyCode::{
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyF, KeyS, KeyW, ShiftLeft, ShiftRight,
};
use bevy::prelude::{
    default, in_state, info, App, AssetEvent, AssetServer, Assets, ButtonInput, Camera, Commands,
    Component, Entity, EventReader, EventWriter, Handle, Image, IntoSystemConfigs, KeyCode,
    NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, Sprite, SpriteBundle,
    TextureAtlas, TextureAtlasBuilder, TextureAtlasLayout, Time, Timer, TimerMode, Transform,
    TransformBundle, UVec2, Update, Vec3, With, Without,
};
use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::prelude::{
    Collider, ImpulseJoint, KinematicCharacterController, NoUserData, RapierDebugRenderPlugin,
    RapierPhysicsPlugin, RigidBody,
};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::vec;

const STARTING_TRANSLATION: Vec3 = Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 1.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 64.0;

const RAW_PLAYER_INITIAL_WIDTH: u32 = 52;
const RAW_PLAYER_INITIAL_HEIGHT: u32 = 52;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //TODO: move setup_animations to animation plugin
            .add_event::<AnimateEvent>()
            .init_resource::<AnimationAssets>()
            .add_systems(OnEnter(AppState::Loading), load_player_assets)
            .add_systems(
                Update,
                check_assets_loading.run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnEnter(AppState::Game), (spawn_player,).chain())
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(OnEnter(AppState::MainMenu), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    stick_camera_to_player,
                    //bound_player_movement,
                    //TODO: move animate to animation plugin
                    animate_v2,
                    listen_for_texture_change,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Run,
}

pub fn load_player_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading Player assets");
    let idle_asset = AnimationAsset {
        atlas_layout: layouts.add(create_atlas(4, 2)),
        texture: asset_server.load("sprites/characters/raw_player/idle-sheet.png"),
        indices: HashMap::from([(Direction::Zero, AnimationIndices::new(0, 6))]),
        is_loaded: false,
    };

    let walk_asset = AnimationAsset {
        atlas_layout: layouts.add(create_atlas(4, 8)),
        texture: asset_server.load("sprites/characters/raw_player/move.png"),
        indices: HashMap::from([
            (Direction::Down, AnimationIndices::new(0, 3)),
            (Direction::DownRight, AnimationIndices::new(4, 7)),
            (Direction::Right, AnimationIndices::new(8, 11)),
            (Direction::UpRight, AnimationIndices::new(12, 15)),
            (Direction::Up, AnimationIndices::new(16, 19)),
            (Direction::UpLeft, AnimationIndices::new(20, 23)),
            (Direction::Left, AnimationIndices::new(24, 27)),
            (Direction::DownLeft, AnimationIndices::new(28, 31)),
        ]),
        is_loaded: false,
    };
    let run_asset = AnimationAsset {
        atlas_layout: layouts.add(create_atlas(4, 8)),
        texture: asset_server.load("sprites/characters/raw_player/move.png"),
        indices: HashMap::from([
            (Direction::Down, AnimationIndices::new(0, 3)),
            (Direction::DownRight, AnimationIndices::new(4, 7)),
            (Direction::Right, AnimationIndices::new(8, 11)),
            (Direction::UpRight, AnimationIndices::new(12, 15)),
            (Direction::Up, AnimationIndices::new(16, 19)),
            (Direction::UpLeft, AnimationIndices::new(20, 23)),
            (Direction::Left, AnimationIndices::new(24, 27)),
            (Direction::DownLeft, AnimationIndices::new(28, 31)),
        ]),
        is_loaded: false,
    };

    commands.insert_resource(AnimationAssets {
        assets: HashMap::from([
            (PlayerState::Idle, idle_asset),
            (PlayerState::Walk, walk_asset),
            (PlayerState::Run, run_asset),
        ]),
    })
}

fn create_atlas(cols: u32, rows: u32) -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        UVec2::new(RAW_PLAYER_INITIAL_WIDTH, RAW_PLAYER_INITIAL_HEIGHT),
        cols,
        rows,
        None,
        None,
    )
}

pub fn check_assets_loading(
    mut assets_event: EventReader<AssetEvent<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_animation_assets: ResMut<AnimationAssets>,
) {
    for event in assets_event.read() {
        for (_, mut asset) in player_animation_assets.assets.iter_mut() {
            if event.is_loaded_with_dependencies(asset.texture.id()) {
                asset.is_loaded = true
            }
        }
    }

    if player_animation_assets.assets.iter().all(|(_, asset)| asset.is_loaded) {
        info!("Assets has been loaded");
        next_state.set(AppState::MainMenu)
    }
}

pub fn spawn_player(mut commands: Commands, player_animations: Res<AnimationAssets>) {
    info!("Spawning Player");
    commands.spawn((
        Movement {
            velocity: 0.0,
            acceleration: 0.0,
            direction: Vec3::ZERO,
        },
        SpriteBundle {
            transform: Transform::from_translation(STARTING_TRANSLATION).with_scale(Vec3::new(5.0, 5.0, 1.0)),
            texture: player_animations.assets.get(&PlayerState::Idle).unwrap().texture.clone(),
            ..default()
        },
        Animation {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
            state: PlayerState::Idle,
            direction: Direction::Zero,
        },
        TextureAtlas {
            layout: player_animations.assets.get(&PlayerState::Walk).unwrap().atlas_layout.clone(),
            index: player_animations.assets.get(&PlayerState::Walk).unwrap().indices.get(&Direction::Up).unwrap().first,
        },
        Controls {
            //TODO: move to resources
            controls_map: HashMap::from([
                (KeyW, ControlledAction::MoveUp),
                (KeyA, ControlledAction::MoveLeft),
                (KeyS, ControlledAction::MoveDown),
                (KeyD, ControlledAction::MoveRight),
                (ArrowUp, ControlledAction::MoveUp),
                (ArrowLeft, ControlledAction::MoveLeft),
                (ArrowDown, ControlledAction::MoveDown),
                (ArrowRight, ControlledAction::MoveRight),
                (ShiftLeft, ControlledAction::Run),
                (KeyF, ControlledAction::Attack),
            ]),
        },
        Collider::cuboid(
            (RAW_PLAYER_INITIAL_WIDTH / 2) as f32,
            (RAW_PLAYER_INITIAL_HEIGHT / 2) as f32,
        ),
        RigidBody::KinematicPositionBased,
        Player {},
    ));
}

pub fn player_movement(
    mut query: Query<Entity, With<Player>>,
    actions: ResMut<Actions>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut animate_event_writer: EventWriter<AnimateEvent>,
) {
    let player_entity = query.single();
    info!("Init actions in player move: {:?}", &actions);

    if actions.current_actions.is_empty() {
        animate_event_writer.send(AnimateEvent::new(PlayerState::Idle, Direction::Zero));
        return;
    }

    if actions.contains_move() {
        if actions.contains_running() {
            let direction = Direction::from_actions(actions.current_actions.clone());
            let move_event = MoveEvent::new(&player_entity, direction, 2.0, PLAYER_SPEED);

            info!("Sending Move event: {:?}", &move_event);
            animate_event_writer.send(AnimateEvent::new(PlayerState::Run, direction));
            move_event_writer.send(move_event);
        } else {
            let direction1 = Direction::from_actions(actions.current_actions.clone());
            let move_event = MoveEvent::new(&player_entity, direction1, 1.0, PLAYER_SPEED);

            info!("Sending Move event: {:?}", &move_event);
            animate_event_writer.send(AnimateEvent::new(PlayerState::Walk, direction1));
            move_event_writer.send(move_event);
        }
    }
}

pub fn stick_camera_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, 2.0 * time.delta_seconds());
    // camera_transform.translation = camera_transform.translation.clamp(
    //     Vec3::new(f32::MIN, -WINDOW_HEIGHT, 0.0),
    //     Vec3::new(f32::MAX, WINDOW_HEIGHT, 0.0),
    // )
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

pub fn bound_player_movement(mut query: Query<&mut Transform, With<Player>>) {
    let mut transform = query.single_mut();

    transform.translation = transform.translation.clamp(
        Vec3::new(f32::MIN, RAW_PLAYER_INITIAL_HEIGHT as f32 / 2.0, 0.0),
        Vec3::new(
            f32::MAX,
            WINDOW_HEIGHT - (RAW_PLAYER_INITIAL_HEIGHT as f32 / 2.0),
            0.0,
        ),
    );
}
