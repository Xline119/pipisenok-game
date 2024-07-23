use bevy::prelude::*;

use crate::animation::animation::{animate, setup_animations};

pub mod animation;

pub struct MineAnimationPlugin;

impl Plugin for MineAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                (
                    setup_animations
                )
            )
            .add_systems(Update, (animate));
    }
}
