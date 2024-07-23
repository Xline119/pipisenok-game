use bevy::prelude::{Query, Res, Time, Transform};

use crate::game::movement::components::{Movement};

pub fn update_position(mut transform_query: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in transform_query.iter_mut() {
        transform.translation += movement.direction * movement.velocity * movement.acceleration * time.delta_seconds()
        //    .clamp(Vec3::new(-WINDOW_WIDTH, -WINDOW_HEIGHT, 0.0), Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0))
        ;
    }
}
