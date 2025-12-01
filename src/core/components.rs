use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment {
    pub segment_index: usize,
}

#[derive(Component)]
pub struct GridPosition(pub IVec2);

#[derive(Component)]
pub struct Apple;
