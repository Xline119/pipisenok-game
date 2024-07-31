use crate::game::game::GameState;
use crate::game::movement::movement::Direction;
use bevy::log::info;
use bevy::prelude::{in_state, App, Component, Entity, Event, EventReader, Handle, Image, IntoSystemConfigs, Plugin, Query, Res, Resource, TextureAtlas, TextureAtlasLayout, Time, Timer, Update, With};
use std::collections::HashMap;
use std::time::Duration;

pub struct PepaAnimationPlugin;

impl Plugin for PepaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimateEvent>()
            .init_resource::<AnimationAssets>()
            .add_systems(
                Update,
                (animate, listen_for_texture_change).run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Default)]
pub struct Animator;

#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Run,
    Attack
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Default)]
pub struct AnimationClip {
    pub state: AnimationState,
    pub direction: Direction,
    pub indices: AnimationIndices,
    pub timer: Timer,
}

pub fn clip_animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationClip, &mut TextureAtlas), With<Animator>>,
) {
    for (mut animation_clip, mut atlas) in query.iter_mut() {
        animation_clip.timer.tick(time.delta());

        if animation_clip.timer.just_finished() {
            atlas.index = match atlas.index {
                idx if idx < animation_clip.indices.first => animation_clip.indices.first,
                idx if idx < animation_clip.indices.last => idx + 1,
                _ => animation_clip.indices.first
            };
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct AnimationAssets {
    pub assets: HashMap<AnimationState, AnimationAsset>,
}

#[derive(Default, Debug)]
pub struct AnimationAsset {
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
    pub indices: HashMap<Direction, AnimationIndices>,
    pub is_loaded: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

#[derive(Component, Default, Debug)]
pub struct Animation {
    pub timer: Timer,
    pub direction: Direction,
    pub state: AnimationState,
    pub next_state: Option<AnimationState>,
    pub next_direction: Option<Direction>,
}

#[derive(Event, Debug, Default)]
pub struct AnimateEvent {
    pub new_state: AnimationState,
    pub new_direction: Direction,
    pub new_timer_duration: Option<Duration>
}

impl AnimateEvent {
    pub fn new(new_state: AnimationState, new_direction: Direction) -> Self {
        Self {
            new_state,
            new_direction,
            new_timer_duration: None
        }
    }

    pub fn new_timer(&mut self, new_timer_duration: Option<Duration>) {
        self.new_timer_duration = new_timer_duration;
    }
}

pub fn animate(
    time: Res<Time>,
    animation_assets: Res<AnimationAssets>,
    mut query: Query<(&mut Animation, &mut TextureAtlas)>,
) {
    let (mut animation, mut atlas) = query.single_mut();
    let asset = animation_assets.assets.get(&animation.state).unwrap();
    animation.timer.tick(time.delta());

    if let Some(indices) = asset.indices.get(&animation.direction) {
        if animation.timer.just_finished() {
            info!("Timer ticked, atlas index: {:?}", atlas.index);

            atlas.index = match atlas.index {
                idx if idx < indices.first => indices.first,
                idx if idx < indices.last => idx + 1,
                _ => indices.first
            };

            // Check if there is a next state and direction to transition to
            if let (Some(next_state), Some(next_direction)) = (animation.next_state.take(), animation.next_direction.take()) {
                animation.state = next_state;
                animation.direction = next_direction;
                let next_asset = animation_assets.assets.get(&animation.state).unwrap();
                let next_indices = next_asset.indices.get(&animation.direction).unwrap();
                atlas.index = next_indices.first;
            }
        }
    }
}

pub fn listen_for_texture_change(
    mut texture_query: Query<(&mut Handle<Image>, &mut TextureAtlas)>,
    mut animation_query: Query<&mut Animation>,
    mut event_reader: EventReader<AnimateEvent>,
    animation_assets: Res<AnimationAssets>,
) {
    for mut event in event_reader.read() {
        info!("get event: {:?}", event);

        let (mut texture, mut atlas) = texture_query.single_mut();
        let mut animation = animation_query.single_mut();
        let asset = animation_assets.assets.get(&event.new_state).unwrap();
        let indices = asset.indices.get(&event.new_direction).unwrap();

        let index = atlas.index.clamp(indices.first, indices.last);

        animation.next_state = Some(event.new_state);
        animation.next_direction = Some(event.new_direction);
        *texture = asset.texture.clone();
        *atlas = TextureAtlas {
            layout: asset.atlas_layout.clone(),
            index,
        };
    }
}
