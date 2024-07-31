use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};

use bevy::prelude::{
    default, App, AssetServer, Commands, Component, Entity, OnEnter, Plugin, Query, Res,
    SpriteBundle, Transform, Vec3, With,
};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_station)
            .add_systems(OnEnter(AppState::MainMenu), despawn_station_location);
    }
}

#[derive(Component)]
pub struct AutumnForestLocation {}

#[derive(Component)]
pub struct StationLocation {
    width: f32,
    height: f32,
}

pub fn spawn_station(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/locations/station/station-new-start-001.png"),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0),
            ..default()
        },
        StationLocation {
            width: 1024.0,
            height: 1024.0,
        },
    ));
}

pub fn spawn_forest_location(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/locations/autumn/forest_full.png"),
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0)
            .with_scale(Vec3::new(0.7, 0.7, 0.0)),
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

pub fn despawn_station_location(
    mut commands: Commands,
    location_query: Query<Entity, With<StationLocation>>,
) {
    if let Ok(location_entity) = location_query.get_single() {
        commands.entity(location_entity).despawn()
    }
}
