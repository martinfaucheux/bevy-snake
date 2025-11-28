use crate::core::Direction;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SnakeHeadDirection {
    pub direction: Direction,
}
