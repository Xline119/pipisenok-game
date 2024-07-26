use bevy::prelude::{
    default, AssetServer, BackgroundColor, BuildChildren, ButtonBundle, Color, Commands,
    DespawnRecursiveExt, Entity, ImageBundle, Query, Res, Text, TextBundle, TextSection, UiImage,
    With,
};

use crate::ui::main_menu::components::{MainMenu, PlayButton};
use crate::ui::main_menu::styles::{get_button_style, get_button_text_style, get_main_menu_style};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despwan_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    return commands
        .spawn((
            ImageBundle {
                style: get_main_menu_style(),
                image: UiImage::new(asset_server.load("images/menu/main_menu.png")),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: BackgroundColor(Color::NONE),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "NEW GAME",
                                get_button_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
}
