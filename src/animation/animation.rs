use bevy::prelude::{App, Assets, Bundle, Commands, Component, Entity, Event, EventReader, info, Mut, Plugin, Query, Res, ResMut, Sprite, Startup, TextureAtlas, TextureAtlasLayout, Time, Timer, Update, UVec2};
use crate::game::movement::movement::Direction;

pub struct PepaAnimationPlugin;

impl Plugin for PepaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Animate>()
            .add_systems(Startup, (setup_animations))
            .add_systems(Update, (animate));
    }
}

#[derive(Event, Debug)]
pub struct Animate {
    pub entity: Entity,
    pub direction: Direction,
    pub animation_indices: AnimationIndices,
}

#[derive(Bundle)]
pub struct Animation {
    pub sheet_props: SheetProps,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

#[derive(Component)]
pub struct SheetProps {
    pub cell_size: UVec2,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub fn setup_animations(
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(Entity, &SheetProps, &AnimationIndices)>,
    mut commands: Commands,
) {
    for (entity, sheet_props, animation_indices) in query.iter() {
        let layout = TextureAtlasLayout::from_grid(
            sheet_props.cell_size,
            sheet_props.cols,
            sheet_props.rows,
            None,
            None,
        );

        commands.entity(entity).insert(TextureAtlas {
            layout: texture_atlas_layouts.add(layout.clone()),
            index: animation_indices.first,
        });
    }
}

//TODO: Make Walk to Idle be more smooth
pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &mut Sprite)>,
    mut event_reader: EventReader<Animate>
) {
    for animate_event in event_reader.read() {
        let (mut timer, mut atlas, mut sprite) = query.get_mut(animate_event.entity).unwrap();
        sprite.flip_x = false;
        timer.0.tick(time.delta());

        if timer.0.finished() {
            set_indexes(atlas, animate_event.animation_indices.last, animate_event.animation_indices.first);
        }

        if animate_event.direction.is_neg_x_axes() {
            sprite.flip_x = true
        }
    }
}

pub fn set_indexes(mut texture_atlas: Mut<TextureAtlas>, max: usize, start: usize) {
    if texture_atlas.index < max {
        texture_atlas.index += 1;
    } else {
        texture_atlas.index = start
    }
}
