use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, default, Query, Res, ResMut, SpriteBundle, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;

use crate::game::enemy::components::Enemy;
use crate::game::enemy::{
    ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES
};
use crate::game::enemy::resources::EnemySpawnTimer;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/balls/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();

    let x_min = 0.0 + (ENEMY_SIZE / 2.0);
    let x_max = window.width() - (ENEMY_SIZE / 2.0);
    let y_min = 0.0 + (ENEMY_SIZE / 2.0);
    let y_max = window.height() - (ENEMY_SIZE / 2.0);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        if transform.translation.x > x_max || transform.translation.x < x_min {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if transform.translation.y > y_max || transform.translation.y < y_min {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            if random::<f32>() > 0.5 {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    settings: PlaybackSettings::ONCE,
                });
            } else {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    settings: PlaybackSettings::ONCE,
                });
            }
        }
    }
}

pub fn bound_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = 0.0 + (ENEMY_SIZE / 2.0);
    let x_max = window.width() - (ENEMY_SIZE / 2.0);
    let y_min = 0.0 + (ENEMY_SIZE / 2.0);
    let y_max = window.height() - (ENEMY_SIZE / 2.0);

    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;

        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max
        }

        enemy_transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    if enemy_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/balls/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}