use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct Movement {
    pub velocity: f32,
    pub acceleration: f32,
    pub direction: Vec3,
}
