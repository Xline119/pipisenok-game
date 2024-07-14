use bevy::prelude::{
    App, Plugin
};

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;

use crate::events::GameOver;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>()
            .add_plugins((PlayerPlugin, EnemyPlugin, StarPlugin));
    }
}