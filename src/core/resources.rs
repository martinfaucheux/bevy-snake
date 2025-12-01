use bevy::prelude::*;

use super::constants::*;

#[derive(Resource)]
pub struct SnakeHeadDirection {
    pub direction: Direction,
}

impl SnakeHeadDirection {
    pub fn default() -> Self {
        Self {
            direction: Direction::Right,
        }
    }
}
