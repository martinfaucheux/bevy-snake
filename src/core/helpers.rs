use super::components::GridPosition;
use super::constants::*;
use bevy::prelude::*;
use rand::Rng;

pub fn grid_pos_to_world_pos(grid_pos: IVec2) -> Vec3 {
    Vec3::new(
        grid_pos.x as f32 * CELL_SIZE - (GRID_SIZE.x as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
        grid_pos.y as f32 * CELL_SIZE - (GRID_SIZE.y as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
        0.0,
    )
}

pub fn world_pos_to_grid_pos(world_pos: Vec3) -> IVec2 {
    IVec2::new(
        (((world_pos.x + (GRID_SIZE.x as f32 * CELL_SIZE) / 2.0) / CELL_SIZE).floor()) as i32,
        (((world_pos.y + (GRID_SIZE.y as f32 * CELL_SIZE) / 2.0) / CELL_SIZE).floor()) as i32,
    )
}

pub fn get_random_grid_pos() -> IVec2 {
    IVec2::new(
        rand::rng().random_range(0..GRID_SIZE.x) as i32,
        rand::rng().random_range(0..GRID_SIZE.y) as i32,
    )
}

/// Gets a random grid position that is not occupied by any entity with a GridPosition component
pub fn get_random_unoccupied_grid_pos(grid_position_query: &Query<&GridPosition>) -> IVec2 {
    let mut position = get_random_grid_pos();
    while grid_position_query.iter().any(|pos| pos.0 == position) {
        position = get_random_grid_pos();
    }
    position
}
