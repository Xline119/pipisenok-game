use bevy::prelude::{Bundle, Component, SpriteBundle, Vec3};

#[derive(Component)]
pub struct Velocity {
    pub value: f32
}

#[derive(Component)]
pub struct Acceleration {
    pub value: f32
}

#[derive(Component)]
pub struct Direction {
    pub value: Vec3
}

impl From<Vec3> for Direction {
    fn from(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MoveableObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub direction: Direction,
    pub model: SpriteBundle
}
