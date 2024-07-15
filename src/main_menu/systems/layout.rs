use bevy::prelude::{AlignItems, AssetServer, BuildChildren, Color, Commands, default, DespawnRecursiveExt, Entity, FlexDirection, ImageBundle, JustifyContent, JustifyText, NodeBundle, Query, Res, Style, Text, TextBundle, TextSection, TextStyle, UiImage, Val, With};
use crate::main_menu::components::MainMenu;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despwan_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    return commands
        .spawn(
            (
                ImageBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("images/menu/main_menu.png")),
                    ..default()
                },
                MainMenu {}
            )
        )
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "G - Start Game          M - Menu".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 33.0,
                                    color: Color::BLACK
                                }
                            }
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            ));
        })
        .id()
}
