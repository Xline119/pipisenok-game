use bevy::prelude::{App, AssetServer, Commands, Component, default,
                    Entity, EventWriter, in_state, IntoSystemConfigs, OnEnter, Plugin, Query,
                    Res, SpriteBundle, Timer, TimerMode, Transform, Update, UVec2, Vec3, With};
use rand::random;

use crate::{
    AppState, WINDOW_HEIGHT, WINDOW_WIDTH
};
use crate::animation::animation::{Animate, Animation, AnimationIndices, AnimationTimer, SheetProps};
use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, Movement};

const WARRIOR_WIDTH: u32 = 128;
const WARRIOR_HEIGHT: u32 = 128;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_warriors)
            .add_systems(OnEnter(AppState::MainMenu), despawn_warriors)
            .add_systems(Update, warrior_movement.run_if(in_state(GameState::Running)));
    }
}

#[derive(Component, Debug)]
pub struct Warrior;

pub fn spawn_warriors(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for i in 0..7 {
        commands.spawn((
            Movement {
                velocity: 0.0,
                acceleration: 0.0,
                direction: Vec3::ZERO,
            },
            SpriteBundle {
                transform: Transform::from_xyz(WINDOW_WIDTH * random::<f32>(), WINDOW_HEIGHT * random::<f32>(), 1.0)
                    .with_scale(Vec3::new(1.5, 1.5, 0.0)),
                texture: asset_server.load("sprites/characters/fighter/Idle.png"),
                ..default()
            },
            Animation {
                sheet_props: SheetProps {
                    cell_size: UVec2::new(WARRIOR_WIDTH, WARRIOR_HEIGHT),
                    rows: 1,
                    cols: 6,
                },
                animation_indices: AnimationIndices {
                    first: 0,
                    last: 5,
                },
                animation_timer: AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
            },
            Warrior {}
        ));
    }
}

pub fn despawn_warriors(mut commands: Commands, query: Query<Entity, With<Warrior>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn()
    }
}

pub fn warrior_movement(
    mut query: Query<Entity, With<Warrior>>,
    mut animate_event_writer: EventWriter<Animate>,
) {
    for entity in query.iter() {
        let animate_event = Animate {
            entity: entity,
            direction: Direction::Zero,
            animation_indices: AnimationIndices {
                first: 0,
                last: 5
            }
        };

        animate_event_writer.send(animate_event);
    }
}
