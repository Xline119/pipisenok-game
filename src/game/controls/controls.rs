use std::cmp::PartialEq;
use std::collections::HashSet;
use std::ops::Add;

use bevy::prelude::{
    App,
    ButtonInput,
    Component,
    Deref,
    in_state,
    info,
    IntoSystemConfigs,
    KeyCode,
    Plugin,
    Query,
    Res,
    ResMut,
    Resource,
    Update
};
use bevy::utils::{
    HashMap,
    info
};

use crate::AppState;
use crate::game::game::GameState;
use crate::game::movement::movement::Direction;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                handle_controls_state.run_if(in_state(GameState::Running))
            );
    }
}

#[derive(Component, Debug)]
pub struct Controls {
    pub controls_map: HashMap<ControlledAction, Vec<KeyCode>>,
    pub state: ControlledAction,
    pub combined_state: Option<CombinedState>,
}

#[derive(Debug)]
pub struct CombinedState {
    //TODO handle state combination like Run + Run Direction or Attack + Attack direction etc.
    state_1: ControlledAction,
    state_2: ControlledAction,
}

#[derive(Hash, Eq, Debug, Copy, Clone)]
pub enum ControlledAction {
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    MoveUpLeft,
    MoveDownRight,
    MoveDownLeft,
    MoveUpRight,
    Run,
    Attack,
    None,
}

impl ControlledAction {
    pub fn get_direction(&self) -> Direction {
        match self {
            ControlledAction::MoveUp => { Direction::Up }
            ControlledAction::MoveLeft => { Direction::Left }
            ControlledAction::MoveDown => { Direction::Down }
            ControlledAction::MoveRight => { Direction::Right }
            ControlledAction::MoveUpLeft => { Direction::UpLeft }
            ControlledAction::MoveDownRight => { Direction::DownRight }
            ControlledAction::MoveDownLeft => { Direction::DownLeft }
            ControlledAction::MoveUpRight => { Direction::UpRight }
            ControlledAction::Run => {
                //TODO: Handle run
                Direction::Zero
            }
            ControlledAction::Attack => {
                //TODO implement attack logic
                Direction::Zero
            }
            ControlledAction::None => { Direction::Zero }
        }
    }

    pub fn is_diagonal_move(&self) -> bool {
        match self {
            ControlledAction::MoveUpLeft => { true }
            ControlledAction::MoveDownRight => { true }
            ControlledAction::MoveDownLeft => { true }
            ControlledAction::MoveUpRight => { true }
            _ => false
        }
    }

    pub fn is_run_action(&self) -> bool {
        match self {
            ControlledAction::Run => { true }
            _ => { false }
        }
    }
}

pub fn handle_controls_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Controls>,
) {
    let pressed_keys: HashSet<KeyCode> = keyboard_input.get_pressed().cloned().collect();
    let mut controls = query.single_mut();
    let cloned_controls_map = controls.controls_map.clone();
    let mut new_state = ControlledAction::None;

    controls.state = ControlledAction::None;
    info!("Pressed keys: {:?}", pressed_keys);

    if pressed_keys.is_empty() {
        return;
    }

    if pressed_keys.len() == 1 {
        let pressed_key = pressed_keys.iter().last().unwrap();

        for (action, mapped_keys) in &cloned_controls_map {
            if !action.is_diagonal_move() && mapped_keys.contains(pressed_key) {
                new_state = *action;
                info!("New action state: {:?}", &new_state);
            }
        }
    }

    if pressed_keys.len() == 2 {
        let first_pressed = pressed_keys.iter().next().unwrap();
        let second_pressed = pressed_keys.iter().last().unwrap();

        for (action, mapped_keys) in &cloned_controls_map {
            if action.is_diagonal_move() && mapped_keys.contains(first_pressed) && mapped_keys.contains(second_pressed) {
                new_state = *action;
                info!("New action state: {:?}", &new_state);
            }
        }
    }

    controls.state = new_state;
    info!("new controls state: {:?}", &controls.state)
}

impl PartialEq for ControlledAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ControlledAction::MoveUp, ControlledAction::MoveUp) => true,
            (ControlledAction::MoveLeft, ControlledAction::MoveLeft) => true,
            (ControlledAction::MoveDown, ControlledAction::MoveDown) => true,
            (ControlledAction::MoveRight, ControlledAction::MoveRight) => true,
            (ControlledAction::MoveUpLeft, ControlledAction::MoveUpLeft) => true,
            (ControlledAction::MoveDownRight, ControlledAction::MoveDownRight) => true,
            (ControlledAction::MoveDownLeft, ControlledAction::MoveDownLeft) => true,
            (ControlledAction::MoveUpRight, ControlledAction::MoveUpRight) => true,
            (ControlledAction::Run, ControlledAction::Run) => true,
            (ControlledAction::None, ControlledAction::None) => true,
            _ => false,
        }
    }
}

impl Controls {
    pub fn is_pressed(&self, controlled_action: ControlledAction) -> bool {
        self.state == controlled_action
    }
}
