use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::prelude::{Commands, default, DespawnRecursiveExt, Entity, Query, Res, Text, TextBundle, TextSection, TextStyle, With};

use crate::ui::in_game::components::ControlsHint;

pub fn spawn_controls_hint(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despwan_controls_hint(mut commands: Commands, query: Query<Entity, With<ControlsHint>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    return commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Pause - 'SPACE' Attack - 'F' Main Menu - 'M' Accelerate - 'Shift'",
                        TextStyle {
                            font: asset_server.load("fonts/Paint-Peel-Cyr.ttf"),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    )],
                    ..default()
                },
                ..default()
            },
            ControlsHint {},
        ))
        .id();
}