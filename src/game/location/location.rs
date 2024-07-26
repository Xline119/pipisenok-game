use bevy::prelude::{App, AssetServer, Commands, Component, default, Entity, OnEnter, Plugin, Query, Res, SpriteBundle, Transform, Vec3, With};
use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_forest_location)
            .add_systems(OnEnter(AppState::MainMenu), despawn_forest_location);
    }
}

#[derive(Component)]
pub struct AutumnForestLocation {}

pub fn spawn_forest_location(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/locations/autumn/forest_full.png"),
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0)
            .with_scale(Vec3::new(0.5, 0.5, 0.0)),
        ..default()
    });
}

pub fn despawn_forest_location(
    mut commands: Commands,
    location_query: Query<Entity, With<AutumnForestLocation>>,
) {
    if let Ok(location_entity) = location_query.get_single() {
        commands.entity(location_entity).despawn()
    }
}
