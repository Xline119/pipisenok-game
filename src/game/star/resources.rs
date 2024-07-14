use bevy::prelude::{Resource, Timer, TimerMode};

const STAR_SPAWN_SECS: f32 = 1.5;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_SECS, TimerMode::Repeating)
        }
    }
}