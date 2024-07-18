use bevy::prelude::{Query, Res, Time, Transform, Vec3};

use crate::game::movement::components::{Acceleration, Direction, Velocity};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn update_position(mut transform_query: Query<(&Velocity, &Direction, &Acceleration, &mut Transform)>, time: Res<Time>) {
    for (velocity, direction, acceleration, mut transform) in transform_query.iter_mut() {
        transform.translation += (direction.value * velocity.value * acceleration.value * time.delta_seconds())
        //    .clamp(Vec3::new(-WINDOW_WIDTH, -WINDOW_HEIGHT, 0.0), Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0))
        ;
    }
}
