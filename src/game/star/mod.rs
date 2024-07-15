use bevy::app::{
    App, Plugin, Startup, Update
};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit};
use resources::StarSpawnTimer;
use systems::*;
use crate::AppState;
use crate::game::GameState;

pub mod components;
pub mod resources;
mod systems;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 14;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)
            .add_systems(
                Update,
                (
                    tick_star_spawn_timer,
                    spawn_stars_over_time
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            );
    }
}