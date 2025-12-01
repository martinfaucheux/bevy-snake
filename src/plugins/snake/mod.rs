use crate::core::*;
use bevy::prelude::*;
use std::collections::HashMap;
pub struct SnakePlugin;

const INIT_SNAKE_SEGMENT_COUNT: i32 = 2;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_snake);
        app.add_systems(Update, move_snake);
    }
}

fn create_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let init_grid_pos = world_pos_to_grid_pos(Vec3::ZERO);
    let shape = meshes.add(Rectangle::new(CELL_SIZE * 0.8, CELL_SIZE * 0.8));
    let material = MeshMaterial2d(materials.add(SNAKE_COLOR));
    commands.spawn((
        Mesh2d(shape.clone()),
        material.clone(),
        Transform {
            translation: grid_pos_to_world_pos(init_grid_pos),
            ..default()
        },
        SnakeHead,
        GridPosition(init_grid_pos),
    ));

    for segment_idx in 1..=INIT_SNAKE_SEGMENT_COUNT {
        let segment_grid_pos = init_grid_pos - IVec2::new(segment_idx, 0);
        commands.spawn((
            Mesh2d(shape.clone()),
            material.clone(),
            Transform {
                translation: grid_pos_to_world_pos(segment_grid_pos),
                ..default()
            },
            SnakeSegment {
                segment_index: segment_idx as usize,
            },
            GridPosition(segment_grid_pos),
        ));
    }
}

fn move_snake(
    snake_head_query: Single<
        (&mut Transform, &mut GridPosition),
        (With<SnakeHead>, Without<SnakeSegment>),
    >,
    mut snake_segment_query: Query<
        (&mut Transform, &mut GridPosition, &SnakeSegment),
        (With<SnakeSegment>, Without<SnakeHead>),
    >,
    mut snake_head_direction: ResMut<SnakeHeadDirection>,
    mut propel_snake_message_reader: MessageReader<PropelSnakeMessage>,
) {
    if propel_snake_message_reader.is_empty() {
        return;
    }
    propel_snake_message_reader.clear();
    snake_head_direction.current_direction = snake_head_direction.recorded_target_direction;

    let (mut snake_transform, mut grid_position) = snake_head_query.into_inner();

    let initial_position = grid_position.0;
    grid_position.0 += snake_head_direction.current_direction.to_ivec2();

    // euclid remaining
    grid_position.0 = grid_position.0.rem_euclid(GRID_SIZE);
    snake_transform.translation = grid_pos_to_world_pos(grid_position.0);

    // map initial positions
    let init_segment_positions: HashMap<usize, IVec2> = snake_segment_query
        .iter()
        .map(|(_, grid_pos, segment)| (segment.segment_index, grid_pos.0))
        .collect();

    // compute target positions
    let mut target_segment_positions: HashMap<usize, IVec2> = HashMap::new();
    target_segment_positions.insert(1, initial_position);
    for segment_idx in 2..=init_segment_positions.len() {
        if let Some(prev_pos) = init_segment_positions.get(&(segment_idx - 1)) {
            target_segment_positions.insert(segment_idx, *prev_pos);
        }
    }

    snake_segment_query
        .iter_mut()
        .for_each(|(mut transform, mut grid_pos, segment)| {
            if let Some(target_pos) = target_segment_positions.get(&segment.segment_index) {
                grid_pos.0 = *target_pos;
                transform.translation = grid_pos_to_world_pos(grid_pos.0);
            }
        });
}
