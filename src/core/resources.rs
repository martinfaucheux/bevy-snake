use bevy::prelude::*;

use super::constants::*;

#[derive(Resource)]
pub struct SnakeHeadDirection {
    // Target direction for the next tick
    pub recorded_target_direction: Direction,
    // Current direction of movement. Stored separately to prevent reversing direction mid-tick
    pub current_direction: Direction,
}

impl SnakeHeadDirection {
    pub fn default() -> Self {
        Self {
            recorded_target_direction: Direction::Right,
            current_direction: Direction::Right,
        }
    }
}
