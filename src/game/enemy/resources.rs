use bevy::prelude::{Resource, Timer, TimerMode};

const ENEMY_SPAWN_SECS: f32 = 3.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub(crate) timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_SECS, TimerMode::Repeating)
        }
    }
}