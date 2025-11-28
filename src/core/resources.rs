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

#[derive(Resource)]
pub struct GameTickTimer(pub Timer);

impl GameTickTimer {
    pub fn default() -> Self {
        Self(Timer::from_seconds(TICK_DURATION, TimerMode::Repeating))
    }
}
