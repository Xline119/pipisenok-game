use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

use bevy::prelude::{in_state, info, App, ButtonInput, Component, Deref, IntoSystemConfigs, KeyCode, Plugin, Query, Res, ResMut, Resource, Update, Event, EventWriter, Entity};

use crate::game::game::GameState;
use crate::game::movement::movement::{Direction, MoveEndEvent};
use crate::AppState;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_event::<ActionEvent>()
            .add_systems(
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

#[derive(Event, Debug)]
pub struct ActionEvent {
    pub actions: HashSet<ControlledAction>
}

impl ActionEvent {
    pub fn new(actions: HashSet<ControlledAction>) -> Self {
        Self {
            actions
        }
    }

    pub fn contains_running(&self) -> bool {
        self.actions.contains(&ControlledAction::Run)
    }

    pub fn contains_move(&self) -> bool {
        self.actions
            .iter()
            .any(|action| action.is_move_action())
    }

    pub fn is_attack(&self) -> bool {
        self.actions.contains(&ControlledAction::Attack)
    }
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
    mut event_writer: EventWriter<ActionEvent>,
    mut move_end_event_writer: EventWriter<MoveEndEvent>,
    mut query: Query<(Entity, &mut Controls)>,
) {
    let pressed_keys: HashSet<KeyCode> = keyboard_input.get_pressed().cloned().collect();
    let released_keys: HashSet<KeyCode> = keyboard_input.get_just_released().cloned().collect();

    let (entity, mut controls) = query.single_mut();
    let mut new_actions = HashSet::new();

    if !released_keys.is_empty() {
        for released_key in released_keys.iter() {
            controls
                .controls_map
                .get(released_key)
                .map(|it| {
                    if it.is_move_action() {
                        move_end_event_writer.send(MoveEndEvent { entity });
                    }
                });
        }
    }

    if !pressed_keys.is_empty() {
        for pressed_key in pressed_keys.iter() {
            controls
                .controls_map
                .get(pressed_key)
                .map(|it| new_actions.insert(*it));
        }

        info!("Sending actions event: {:?}", &new_actions);
        event_writer.send(ActionEvent::new(new_actions));
    }
}
