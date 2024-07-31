use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

use bevy::prelude::{
    in_state, info, App, ButtonInput, Component, Deref, IntoSystemConfigs, KeyCode, Plugin, Query,
    Res, ResMut, Resource, Update,
};

use crate::game::game::GameState;
use crate::game::movement::movement::Direction;
use crate::AppState;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            handle_controls_state.run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component, Debug)]
pub struct Controls {
    pub controls_map: HashMap<KeyCode, ControlledAction>,
}

#[derive(Resource, Debug, Default, Deref)]
pub struct Actions {
    pub current_actions: HashSet<ControlledAction>,
}

#[derive(Hash, Eq, Debug, Copy, Clone, Default)]
pub enum ControlledAction {
    #[default]
    None,
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    Run,
    Attack,
}

impl Actions {
    pub fn contains_running(&self) -> bool {
        self.current_actions.contains(&ControlledAction::Run)
    }

    pub fn contains_move(&self) -> bool {
        self.current_actions
            .iter()
            .any(|action| action.is_move_action())
    }
}

impl PartialEq for ControlledAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ControlledAction::MoveUp, ControlledAction::MoveUp) => true,
            (ControlledAction::MoveLeft, ControlledAction::MoveLeft) => true,
            (ControlledAction::MoveDown, ControlledAction::MoveDown) => true,
            (ControlledAction::MoveRight, ControlledAction::MoveRight) => true,
            (ControlledAction::Run, ControlledAction::Run) => true,
            (ControlledAction::Attack, ControlledAction::Attack) => true,
            (ControlledAction::None, ControlledAction::None) => true,
            _ => false,
        }
    }
}

impl ControlledAction {
    const MOVE_ACTIONS: [Self; 4] = [
        ControlledAction::MoveUp,
        ControlledAction::MoveDown,
        ControlledAction::MoveLeft,
        ControlledAction::MoveRight,
    ];

    pub fn get_direction(&self) -> Direction {
        match self {
            ControlledAction::MoveUp => Direction::Up,
            ControlledAction::MoveLeft => Direction::Left,
            ControlledAction::MoveDown => Direction::Down,
            ControlledAction::MoveRight => Direction::Right,
            ControlledAction::Run => Direction::Zero,
            ControlledAction::Attack => Direction::Zero,
            ControlledAction::None => Direction::Zero,
        }
    }

    pub fn is_move_action(&self) -> bool {
        Self::MOVE_ACTIONS.contains(self)
    }

    pub fn is_run_action(&self) -> bool {
        *self == ControlledAction::Run
    }
}

pub fn handle_controls_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut actions: ResMut<Actions>,
    mut query: Query<&mut Controls>,
) {
    let pressed_keys: HashSet<KeyCode> = keyboard_input.get_pressed().cloned().collect();
    let released_keys: HashSet<KeyCode> = keyboard_input.get_just_released().cloned().collect();
    let mut controls = query.single_mut();
    info!("Pressed keys: {:?}", pressed_keys);

    for released_key in released_keys.iter() {
        controls
            .controls_map
            .get(released_key)
            .map(|it| actions.current_actions.remove(it));
    }

    for pressed_key in pressed_keys.iter() {
        controls
            .controls_map
            .get(pressed_key)
            .map(|it| actions.current_actions.insert(*it));
    }

    info!("Saved actions: {:?}", actions)
}
