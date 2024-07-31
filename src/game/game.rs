use bevy::prelude::{
    default, in_state, info, App, AppExtStates, ButtonInput, Camera2dBundle, Commands,
    IntoSystemConfigs, KeyCode, NextState, Plugin, Res, ResMut, Startup, State, States, Transform,
    Update,
};

use crate::animation::animation::PepaAnimationPlugin;
use crate::game::controls::controls::ControlsPlugin;
use crate::game::movement::movement::MovementPlugin;
use crate::game::npc::npc::NpcPlugin;
use crate::game::player::player::PlayerPlugin;
use crate::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct GamePlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Paused,
    Running,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((
                PlayerPlugin,
                MovementPlugin,
                ControlsPlugin,
                //NpcPlugin
            ))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (toggle_pause,).run_if(in_state(AppState::Game)));
    }
}
pub fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameState::Paused => {
                next_state.set(GameState::Running);
                info!("Game resumed")
            }
            GameState::Running => {
                next_state.set(GameState::Paused);
                info!("Game paused")
            }
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 1.0),
        ..default()
    });
}
