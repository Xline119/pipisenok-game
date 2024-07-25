use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;

use crate::AppState;
use crate::game::GameState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Move>()
            .add_systems(
                Update,
                (update_position)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            );
    }
}

#[derive(Component, Debug)]
pub struct Movement {
    pub velocity: f32,
    pub acceleration: f32,
    pub direction: Vec3,
}

#[derive(Event, Debug)]
pub struct Move {
    pub entity: Entity,
    pub direction: Direction,
    pub acceleration: f32,
    pub speed: f32
}

#[derive(Default, Copy, Clone, Debug, Eq, Hash)]
pub enum Direction {
    #[default]
    Zero,
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up) => true,
            (Direction::Down, Direction::Down) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            (Direction::UpRight, Direction::UpRight) => true,
            (Direction::UpLeft, Direction::UpLeft) => true,
            (Direction::DownRight, Direction::DownRight) => true,
            (Direction::DownLeft, Direction::DownLeft) => true,
            (Direction::Zero, Direction::Zero) => true,
            _ => false,
        }
    }
}

impl Move {
    pub fn get_direction_vec(&self) -> Vec3 {
        match self.direction {
            Direction::Up => { Vec3::Y }
            Direction::Down => { Vec3::NEG_Y }
            Direction::Right => { Vec3::X }
            Direction::Left => { Vec3::NEG_X }
            Direction::UpRight => { Vec3::new(1.0, 1.0, 0.0) }
            Direction::UpLeft => { Vec3::new(-1.0, 1.0, 0.0) }
            Direction::DownRight => { Vec3::new(1.0, -1.0, 0.0) }
            Direction::DownLeft => { Vec3::new(-1.0, -1.0, 0.0) }
            _ => { Vec3::ZERO }
        }
    }
}

// pub fn update_direction(
//     mut event_reader: EventReader<Move>,
//     mut transform_query: Query<(&Movement, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     for mut move_event in event_reader.read() {
//         let (movement, mut transform) = transform_query.get_mut(move_event.entity).unwrap();
//         transform.translation += movement.direction * movement.velocity * movement.acceleration * time.delta_seconds();
//     }
// }

pub fn update_position(
    mut event_reader: EventReader<Move>,
    mut transform_query: Query<(&Movement, &mut Transform)>,
    time: Res<Time>,
) {
    //info!("Updating position");
    for mut move_event in event_reader.read() {
        let (movement, mut transform) = transform_query.get_mut(move_event.entity).unwrap();

        info!("Init transform: {}, of: {}", move_event.entity, transform.translation);
        let mut direction = move_event.get_direction_vec();

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        info!("Direction: {:?}", direction);

        transform.translation += direction * move_event.speed * move_event.acceleration * time.delta_seconds();
        info!("New transform: {}", transform.translation)
    }
}
