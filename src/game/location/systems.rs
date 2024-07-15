use bevy::asset::AssetServer;
use bevy::prelude::{Commands, default, Entity, Query, Res, SpriteBundle, Transform, Vec3, With};
use bevy::window::{PrimaryWindow, Window};
use crate::game::location::components::AutumnForestLocation;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn spawn_forest_location(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/locations/autumn/forest_full.png"),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0)
                .with_scale(Vec3::new(0.5, 0.5, 0.0)),
            ..default()
        }
    ));
}

pub fn despawn_forest_location(
    mut commands: Commands,
    location_query: Query<Entity, With<AutumnForestLocation>>
) {
    if let Ok(location_entity) = location_query.get_single() {
        commands.entity(location_entity).despawn()
    }
}
