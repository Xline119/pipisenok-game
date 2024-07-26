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
    pub direction: Direction
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
    Run(Direction),
    Attack,
    None,
}

impl ControlledAction {
    const MOVE_ACTIONS: [Self; 8] = [
        ControlledAction::MoveUp,
        ControlledAction::MoveDown,
        ControlledAction::MoveLeft,
        ControlledAction::MoveRight,
        ControlledAction::MoveUpLeft,
        ControlledAction::MoveDownRight,
        ControlledAction::MoveDownLeft,
        ControlledAction::MoveUpRight,
    ];

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
            ControlledAction::Run(direction) => { *direction }
            ControlledAction::Attack => {
                //TODO implement attack logic
                Direction::Zero
            }
            ControlledAction::None => { Direction::Zero }
        }
    }

    pub fn is_move(&self) -> bool {
        match self {
            ControlledAction::MoveUp => { true }
            ControlledAction::MoveDown => { true }
            ControlledAction::MoveLeft => { true }
            ControlledAction::MoveRight => { true }
            _ => false
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
            ControlledAction::Run(Direction::Up) => { true }
            ControlledAction::Run(Direction::Left) => { true }
            ControlledAction::Run(Direction::Down) => { true }
            ControlledAction::Run(Direction::Right) => { true }
            _ => { false }
        }
    }

    pub fn is_diagonal_run_action(&self) -> bool {
        match self {
            ControlledAction::Run(Direction::UpLeft) => { true }
            ControlledAction::Run(Direction::DownLeft) => { true }
            ControlledAction::Run(Direction::UpRight) => { true }
            ControlledAction::Run(Direction::DownRight) => { true }
            _ => { false }
        }
    }

    pub fn is_move_action(&self) -> bool {
        Self::MOVE_ACTIONS.contains(self)
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
            if action.is_move() && mapped_keys.contains(pressed_key) {
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

            if action.is_run_action() && mapped_keys.contains(first_pressed) && mapped_keys.contains(second_pressed) {
                new_state = *action;
                info!("New action state: {:?}", &new_state);
            }
        }
    }

    if pressed_keys.len() == 3 {
        let first_pressed = pressed_keys.iter().next().unwrap();
        let second_pressed = pressed_keys.iter().nth(1).unwrap();
        let third_pressed = pressed_keys.iter().nth(2).unwrap();

        for (action, mapped_keys) in &cloned_controls_map {
            if action.is_diagonal_run_action() && mapped_keys.contains(first_pressed) && mapped_keys.contains(second_pressed) && mapped_keys.contains(third_pressed) {
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
            (ControlledAction::Run(first_direction), ControlledAction::Run(second_direction)) => first_direction == second_direction,
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
