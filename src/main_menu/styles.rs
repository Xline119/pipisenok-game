use bevy::prelude::{AlignItems, AssetServer, Color, FlexDirection, JustifyContent, Res, Style, TextStyle, UiRect, Val};

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn get_main_menu_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..Style::DEFAULT
    }
}

pub fn get_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(300.0),
        height: Val::Px(80.0),
        ..Style::DEFAULT
    }
}

pub fn get_main_menu_image_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
        ..Style::DEFAULT
    }
}

pub fn get_image_style() -> Style {
    Style {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
        ..Style::DEFAULT
    }
}

pub fn get_title_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(300.0),
        height: Val::Px(120.0),
        ..Style::DEFAULT
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Paint-Peel-Cyr.ttf"),
        font_size: 64.0,
        color: Color::BLACK,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Paint-Peel-Cyr.ttf"),
        font_size: 50.0,
        color: Color::BLACK,
    }
}