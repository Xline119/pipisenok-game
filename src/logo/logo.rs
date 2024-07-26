use bevy::prelude::*;

use crate::animation::animation::{Animation, AnimationIndices, AnimationTimer, SheetProps};
use crate::{AppState, WINDOW_WIDTH};

const LOGO_WIDTH: f32 = 566.0;
const LOGO_HEIGHT: f32 = 68.0;

const LOGO_DURATION: f32 = 10.0;

pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LogoTimer>()
            .add_systems(Startup, show_logo.run_if(in_state(AppState::Logo)))
            .add_systems(Update, handle_logo_end.run_if(in_state(AppState::Logo)));
    }
}

#[derive(Component)]
pub struct Logo;

#[derive(Resource)]
pub struct LogoTimer(pub Timer);

impl Default for LogoTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(LOGO_DURATION, TimerMode::Once))
    }
}

pub fn show_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, 1.0),
            texture: asset_server.load("images/pipisenok-studios-logo.png"),
            ..default()
        },
        AudioBundle {
            source: asset_server.load("audio/pipisenok-logo.wav"),
            settings: PlaybackSettings::ONCE,
        },
        Animation {
            sheet_props: SheetProps {
                cell_size: UVec2::new((LOGO_WIDTH / 8.0) as u32, LOGO_HEIGHT as u32),
                rows: 1,
                cols: 8,
            },
            //animation_direction: AnimationDirection::Still,
            animation_indices: AnimationIndices { first: 0, last: 7 },
            animation_timer: AnimationTimer(Timer::from_seconds(0.625, TimerMode::Once)),
        },
        Logo {},
    ));
}

pub fn handle_logo_end(
    mut logo_timer: ResMut<LogoTimer>,
    mut commands: Commands,
    query: Query<Entity, With<Logo>>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    logo_timer.0.tick(time.delta());
    if logo_timer.0.finished() {
        next_state.set(AppState::MainMenu);
        let entity = query.single();

        commands.entity(entity).despawn();
    }
}
