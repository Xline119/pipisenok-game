use bevy::prelude::{
    default, in_state, info, App, AssetServer, Commands, Component, Entity, EventWriter,
    IntoSystemConfigs, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, SpriteBundle, Time,
    Timer, TimerMode, Transform, UVec2, Update, Vec3, With,
};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{GravityScale, KinematicCharacterController, RigidBody};
use rand::random;

use crate::animation::animation::AnimateEvent;
use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, MoveEvent, Movement};
use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};

const WARRIOR_WIDTH: u32 = 128;
const WARRIOR_HEIGHT: u32 = 128;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WarriorDirection>()
            .init_resource::<WarriorDirectionTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_warriors)
            .add_systems(OnExit(AppState::Game), despawn_warriors)
            .add_systems(OnEnter(AppState::MainMenu), despawn_warriors)
            .add_systems(
                Update,
                (
                    //warrior_movement,
                    change_warrior_direction
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component, Debug)]
pub struct Warrior;

#[derive(Resource, Default)]
pub struct WarriorDirection {
    direction: Direction,
}

#[derive(Resource)]
pub struct WarriorDirectionTimer(Timer);

impl Default for WarriorDirectionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

pub fn spawn_warriors(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Movement {
            velocity: 0.0,
            acceleration: 0.0,
            direction: Vec3::ZERO,
        },
        SpriteBundle {
            transform: Transform::from_xyz(
                WINDOW_WIDTH * random::<f32>(),
                WINDOW_HEIGHT * random::<f32>(),
                1.0,
            )
            .with_scale(Vec3::new(1.5, 1.5, 0.0)),
            texture: asset_server.load("sprites/characters/fighter/idle-walk.png"),
            ..default()
        },
        Collider::cuboid((60 / 2) as f32, (55) as f32),
        RigidBody::Dynamic,
        GravityScale(0.0),
        //KinematicCharacterController::default(),
        Warrior {},
    ));
}

pub fn despawn_warriors(mut commands: Commands, query: Query<Entity, With<Warrior>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn()
    }
}

pub fn change_warrior_direction(
    mut warrior_direction: ResMut<WarriorDirection>,
    mut timer: ResMut<WarriorDirectionTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        warrior_direction.direction = Direction::Right;
        info!(
            "New warrior direction: {:?}",
            warrior_direction.direction.get_direction_vec()
        )
    }
}

pub fn warrior_movement(
    mut query: Query<Entity, With<Warrior>>,
    mut animate_event_writer: EventWriter<AnimateEvent>,
    mut move_event_writer: EventWriter<MoveEvent>,
    warrior_direction: Res<WarriorDirection>,
) {
    info!(
        "Warrior direction from res: {:?}",
        warrior_direction.direction
    );
    for entity in query.iter() {
        move_event_writer.send(MoveEvent::new(
            &entity,
            warrior_direction.direction,
            50.0,
            1.0,
        ));
    }
}
