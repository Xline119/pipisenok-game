use bevy::app::Update;
use bevy::prelude::{App, Plugin, Startup};
use crate::game::player::systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, bound_player_movement, enemy_hit_player, player_collect_star));
    }
}