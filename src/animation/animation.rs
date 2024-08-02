use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::{App, Component, Entity, Event, EventReader, Handle, Image, in_state, info, IntoSystemConfigs, Plugin, Query, Res, Resource, TextureAtlas, TextureAtlasLayout, Time, Timer, TimerMode, Update};
use bevy::utils::info;
use crate::game::game::GameState;
use crate::game::movement::movement::Direction;

pub struct PepaAnimationPlugin;

impl Plugin for PepaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClipChangeEvent>()
            .init_resource::<AnimationLibrary>()
            .add_systems(
                Update,
                (
                    animate_clip,
                    change_animation_clip
                ).run_if(in_state(GameState::Running)),
            );
    }
}

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
    pub indices: AnimationIndices,
    pub timer: Timer,
}

impl AnimationClip {
    pub fn new(indices: AnimationIndices, timer_mills: u64, timer_mode: TimerMode) -> Self {
        Self {
            indices,
            timer: Timer::from_seconds(Duration::from_millis(timer_mills).as_secs_f32(), timer_mode),
        }
    }

    pub fn new_with_timer(indices: AnimationIndices, timer: Timer) -> Self {
        Self {
            indices,
            timer,
        }
    }
}

#[derive(Debug, Default)]
pub struct AnimationClipResource {
    pub indices: AnimationIndices,
    pub timer: Timer,
}

impl AnimationClipResource {
    pub fn new(indices: AnimationIndices, timer_mills: u64, timer_mode: TimerMode) -> Self {
        Self {
            indices,
            timer: Timer::from_seconds(Duration::from_millis(timer_mills).as_secs_f32(), timer_mode),
        }
    }
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

#[derive(Event, Debug)]
pub struct ClipChangeEvent {
    pub entity: Entity,
    pub new_state: AnimationState,
    pub new_direction: Direction,
}

impl ClipChangeEvent {
    pub fn new(entity: &Entity, new_state: AnimationState, new_direction: Direction) -> Self {
        Self {
            entity: *entity,
            new_state,
            new_direction,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct AnimationResource {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl AnimationResource {
    pub fn new(texture: Handle<Image>, atlas_layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            texture,
            atlas_layout,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct AnimationLibrary {
    pub clips: HashMap<(AnimationState, Direction), (AnimationClipResource, AnimationResource)>,
}

pub fn animate_clip(
    time: Res<Time>,
    mut query: Query<(&mut AnimationClip, &mut TextureAtlas)>,
) {
    for (mut animation_clip, mut atlas) in query.iter_mut() {
        animation_clip.timer.tick(time.delta());
        info!("Timer tick atlas index: {:?}", &atlas.index);

        if animation_clip.timer.just_finished() {
            atlas.index = match atlas.index {
                idx if idx < animation_clip.indices.first => animation_clip.indices.first,
                idx if idx < animation_clip.indices.last => idx + 1,
                _ => animation_clip.indices.first
            };
        }
    }
}

pub fn change_animation_clip(
    mut query: Query<(&mut AnimationClip, &mut TextureAtlas, &mut Handle<Image>)>,
    mut event_reader: EventReader<ClipChangeEvent>,
    animation_library: Res<AnimationLibrary>,
) {
    for event in event_reader.read() {
        info!("Change animation clip event: {:?}", event);
        if let Ok((mut animation_clip, mut texture_atlas, mut texture)) = query.get_mut(event.entity) {
            if let Some((new_clip, resource)) = animation_library.clips.get(&(event.new_state, event.new_direction)) {
                if animation_clip.timer.just_finished() {
                    info!("Changing clip to: {:?} with resource: {:?}", &new_clip, &resource);

                    animation_clip.indices = new_clip.indices.clone();
                    animation_clip.timer = new_clip.timer.clone();
                    *texture = resource.texture.clone();
                    *texture_atlas = TextureAtlas {
                        layout: resource.atlas_layout.clone(),
                        index: texture_atlas.index,
                    };
                }
            } else {
                panic!("No clip found for state: {:?} and direction: {:?}", event.new_state, event.new_direction);
            }
        }
    }
}
