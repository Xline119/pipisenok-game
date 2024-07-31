use crate::game::game::GameState;
use crate::game::movement::movement::Direction;
use crate::game::player::player::PlayerState;
use bevy::log::info;
use bevy::prelude::{
    in_state, App, Component, Entity, Event, EventReader, Handle, Image, IntoSystemConfigs, Plugin,
    Query, Res, Resource, TextureAtlas, TextureAtlasLayout, Time, Timer, Update,
};
use std::collections::HashMap;

pub struct PepaAnimationPlugin;

impl Plugin for PepaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimateEvent>()
            .init_resource::<AnimationAssets>()
            .add_systems(
                Update,
                (animate_v2, listen_for_texture_change).run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Resource, Default, Debug)]
pub struct AnimationAssets {
    pub assets: HashMap<PlayerState, AnimationAsset>,
}

#[derive(Default, Debug)]
pub struct AnimationAsset {
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
    pub indices: HashMap<Direction, AnimationIndices>,
    pub is_loaded: bool,
}

#[derive(Default, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub direction: Direction,
    pub state: PlayerState,
}

#[derive(Event, Debug, Default)]
pub struct AnimateEvent {
    pub new_state: PlayerState,
    pub new_direction: Direction,
}

impl AnimateEvent {
    pub fn new(new_state: PlayerState, new_direction: Direction) -> Self {
        Self {
            new_state,
            new_direction,
        }
    }
}

pub fn animate_v2(
    mut query: Query<(&mut Animation, &mut TextureAtlas)>,
    animation_assets: Res<AnimationAssets>,
    time: Res<Time>,
) {
    let (mut animation, mut atlas) = query.single_mut();
    let asset = animation_assets.assets.get(&animation.state).unwrap();
    animation.timer.tick(time.delta());

    if let Some(indices) = asset.indices.get(&animation.direction) {
        if animation.timer.just_finished() {
            info!("Timer ticked, atlas index: {:?}", atlas.index);
            if atlas.index < indices.first {
                atlas.index = indices.first;
            }

            if atlas.index < indices.last {
                atlas.index += 1
            } else {
                atlas.index = indices.first
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
        let mut index = atlas.index;
        let indices = asset.indices.get(&event.new_direction).unwrap();

        if index < indices.first {
            index = indices.first
        }

        if index > indices.last {
            index = indices.last
        }

        animation.state = event.new_state;
        animation.direction = event.new_direction;
        *texture = asset.texture.clone();
        *atlas = TextureAtlas {
            layout: asset.atlas_layout.clone(),
            index,
        };
    }
}
