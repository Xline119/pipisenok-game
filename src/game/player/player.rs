use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::vec;

use bevy::asset::ErasedAssetLoader;
use bevy::audio::CpalSample;
use bevy::prelude::{
    App, AssetEvent, Assets, AssetServer, ButtonInput, Camera, Commands, Component, default, Entity,
    EventReader, EventWriter, Handle, Image, in_state, info, IntoSystemConfigs, KeyCode,
    NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, Sprite, SpriteBundle,
    TextureAtlas, TextureAtlasBuilder, TextureAtlasLayout, Time, Timer, TimerMode, Transform,
    TransformBundle, Update, UVec2, Vec3, With, Without,
};
use bevy::prelude::KeyCode::{
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyA, KeyD, KeyF, KeyS, KeyW, ShiftLeft, ShiftRight,
};
use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::prelude::{
    Collider, ImpulseJoint, KinematicCharacterController, NoUserData, RapierDebugRenderPlugin,
    RapierPhysicsPlugin, RigidBody,
};

use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::animation::animation::{animate_clip, AnimationClip, AnimationClipResource, AnimationIndices, AnimationLibrary, AnimationResource, AnimationState, change_animation_clip, ClipChangeEvent, PepaAnimationPlugin};
use crate::game::controls::controls::{ActionEndEvent, ActionEvent, Actions, ControlledAction, Controls};
use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, MoveEndEvent, MoveEvent};

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
            .add_event::<ClipChangeEvent>()
            .init_resource::<AnimationLibrary>()
            .init_resource::<PlayerAsset>()
            .init_resource::<PlayerAssets>()
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
                    //TODO: move animate to animation plugin
                    animate_clip,
                    change_animation_clip
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Debug, Default, Eq, PartialEq, Hash)]
pub struct PlayerAsset {
    pub texture: Handle<Image>,
    pub is_loaded: bool,
}

#[derive(Resource, Default, Debug, Eq, PartialEq)]
pub struct PlayerAssets {
    pub assets: Vec<PlayerAsset>,
}

pub fn load_player_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading Player assets");

    let idle_texture = asset_server.load("sprites/characters/raw_player/idle-sheet.png");
    let move_texture = asset_server.load("sprites/characters/raw_player/move.png");
    let attack_texture = asset_server.load("sprites/characters/raw_player/attack.png");
    let idle_atlas = layouts.add(create_atlas(4, 2));
    let move_atlas = layouts.add(create_atlas(4, 8));
    let run_atlas = layouts.add(create_atlas(4, 8));
    let attack_atlas = layouts.add(create_atlas(4, 8));

    let create_resource = |indices: AnimationIndices, texture: Handle<Image>, atlas: Handle<TextureAtlasLayout>, timer_mills: u64| {
        (
            AnimationClipResource::new(indices, timer_mills, TimerMode::Repeating),
            AnimationResource::new(texture.clone(), move_atlas.clone()),
        )
    };

    let library = AnimationLibrary {
        clips: HashMap::from([
            //Idle
            ((AnimationState::Idle, Direction::Zero), create_resource(AnimationIndices::new(0, 6), idle_texture.clone(), idle_atlas.clone(), 125)),
            //Walk
            ((AnimationState::Walk, Direction::Down), create_resource(AnimationIndices::new(0, 3), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::DownRight), create_resource(AnimationIndices::new(4, 7), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::Right), create_resource(AnimationIndices::new(8, 11), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::UpRight), create_resource(AnimationIndices::new(12, 15), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::Up), create_resource(AnimationIndices::new(16, 19), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::UpLeft), create_resource(AnimationIndices::new(20, 23), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::Left), create_resource(AnimationIndices::new(24, 27), move_texture.clone(), move_atlas.clone(), 125)),
            ((AnimationState::Walk, Direction::DownLeft), create_resource(AnimationIndices::new(28, 31), move_texture.clone(), move_atlas.clone(), 125)),
            //Run
            ((AnimationState::Run, Direction::Down), create_resource(AnimationIndices::new(0, 3), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::DownRight), create_resource(AnimationIndices::new(4, 7), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::Right), create_resource(AnimationIndices::new(8, 11), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::UpRight), create_resource(AnimationIndices::new(12, 15), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::Up), create_resource(AnimationIndices::new(16, 19), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::UpLeft), create_resource(AnimationIndices::new(20, 23), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::Left), create_resource(AnimationIndices::new(24, 27), move_texture.clone(), run_atlas.clone(), 125)),
            ((AnimationState::Run, Direction::DownLeft), create_resource(AnimationIndices::new(28, 31), move_texture.clone(), run_atlas.clone(), 125)),
            //Attack
            ((AnimationState::Attack, Direction::Zero), create_resource(AnimationIndices::new(0, 3), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::Down), create_resource(AnimationIndices::new(0, 3), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::DownRight), create_resource(AnimationIndices::new(4, 7), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::Right), create_resource(AnimationIndices::new(8, 11), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::UpRight), create_resource(AnimationIndices::new(12, 15), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::Up), create_resource(AnimationIndices::new(16, 19), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::UpLeft), create_resource(AnimationIndices::new(20, 23), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::Left), create_resource(AnimationIndices::new(24, 27), attack_texture.clone(), attack_atlas.clone(), 200)),
            ((AnimationState::Attack, Direction::DownLeft), create_resource(AnimationIndices::new(28, 31), attack_texture.clone(), attack_atlas.clone(), 200)),
        ]),
    };

    commands.insert_resource(PlayerAssets {
        assets: vec![
            PlayerAsset {
                texture: idle_texture,
                is_loaded: false,
            },
            PlayerAsset {
                texture: move_texture,
                is_loaded: false,
            },
        ]
    });
    commands.insert_resource(library);
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
    mut player_animation_assets: ResMut<PlayerAssets>,
) {
    for event in assets_event.read() {
        for mut asset in player_animation_assets.assets.iter_mut() {
            if event.is_loaded_with_dependencies(asset.texture.id()) {
                asset.is_loaded = true
            }
        }
    }

    if player_animation_assets.assets.iter().all(|asset| asset.is_loaded) {
        info!("Assets has been loaded");
        next_state.set(AppState::MainMenu)
    }
}

pub fn spawn_player(mut commands: Commands, animation_library: Res<AnimationLibrary>) {
    info!("Spawning Player");
    let (clip, resource) = animation_library.clips.get(&(AnimationState::Idle, Direction::Zero)).unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(STARTING_TRANSLATION).with_scale(Vec3::new(5.0, 5.0, 1.0)),
            texture: resource.texture.clone(),
            ..default()
        },
        AnimationClip::new_with_timer(clip.indices.clone(), clip.timer.clone()),
        TextureAtlas {
            layout: resource.atlas_layout.clone(),
            index: clip.indices.first,
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
            (RAW_PLAYER_INITIAL_WIDTH / 4) as f32,
            (RAW_PLAYER_INITIAL_HEIGHT / 4) as f32,
        ),
        RigidBody::KinematicPositionBased,
        Player {},
    ));
}

pub fn player_movement(
    mut query: Query<Entity, With<Player>>,
    mut event_reader: EventReader<ActionEvent>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut clip_event_writer: EventWriter<ClipChangeEvent>,
) {
    let mut prev_event = None;
    for event in event_reader.read() {
        let player_entity = query.single();
        info!("Get event: {:?}", event);

        if prev_event == Some(event) {
            return;
        }

        if event.is_idle() {
            clip_event_writer.send(ClipChangeEvent::new(&player_entity, AnimationState::Idle, Direction::Zero));
            prev_event = Some(event);
            return;
        }

        if event.is_attack() {
            let attack_event = ClipChangeEvent::new(&player_entity, AnimationState::Attack, Direction::Zero);
            let move_event = MoveEvent::new(&player_entity, Direction::Zero, 1.0, PLAYER_SPEED);
            info!("Sending Move event: {:?} and ClipChange event: {:?}", &move_event, &attack_event);

            move_event_writer.send(move_event);
            clip_event_writer.send(attack_event);
            prev_event = Some(event);
            return;
        }

        if event.contains_move() {
            let direction = Direction::from_actions(event.actions.clone());

            if event.contains_attack() {
                let attack_event = ClipChangeEvent::new(&player_entity, AnimationState::Attack, direction);
                let move_event = MoveEvent::new(&player_entity, direction, 1.0, PLAYER_SPEED);
                info!("Sending Move event: {:?} and ClipChange event: {:?}", &move_event, &attack_event);

                move_event_writer.send(move_event);
                clip_event_writer.send(attack_event);
                prev_event = Some(event);
                return;
            }

            let (animation_state, speed_multiplier) = if event.contains_running() {
                (AnimationState::Run, 2.0)
            } else {
                (AnimationState::Run, 1.0)
            };

            let clip_event = ClipChangeEvent::new(&player_entity, animation_state, direction);
            let move_event = MoveEvent::new(&player_entity, direction, speed_multiplier, PLAYER_SPEED);

            info!("Sending Move event: {:?} and ClipChange event: {:?}", &move_event, &clip_event);
            move_event_writer.send(move_event);
            clip_event_writer.send(clip_event);
        }

        prev_event = Some(event);
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
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}
