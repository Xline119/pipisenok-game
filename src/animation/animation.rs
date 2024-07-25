use bevy::prelude::*;

pub struct PepaAnimationPlugin;

impl Plugin for PepaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                (
                    setup_animations
                )
            )
            .add_systems(Update, (animate));
    }
}

#[derive(Bundle)]
pub struct Animation {
    pub sheet_props: SheetProps,
    //pub animation_direction: AnimationDirection,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

#[derive(Component)]
pub struct SheetProps {
    pub cell_size: UVec2,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug, PartialEq, Eq, Clone, Default)]
pub enum AnimationDirection {
    #[default]
    Still,
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

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

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &AnimationIndices, &mut TextureAtlas)>,
) {
    for (mut timer, indices, mut texture_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            set_indexes(texture_atlas, indices.last, indices.first);
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
