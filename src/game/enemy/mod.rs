use bevy::app::Update;
use bevy::prelude::{App, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Startup};

use resources::EnemySpawnTimer;
use systems::*;
use crate::AppState;
use crate::game::GameState;

pub mod components;
pub mod resources;
mod systems;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(OnEnter(AppState::MainMenu), despawn_enemies)
            .add_systems(
                Update,
                (
                    bound_enemy_movement.before(enemy_movement),
                    enemy_movement,
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
            );
    }
}