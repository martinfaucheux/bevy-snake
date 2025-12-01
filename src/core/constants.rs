use bevy::prelude::*;

pub const TICK_DURATION: f32 = 0.5;
pub const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const CELL_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);

pub const CELL_BORDER_THICKNESS: f32 = 2.0;

pub const SNAKE_COLOR: Color = Color::srgb(0.3882353, 0.8117647, 0.6431373); // #63cfa4
pub const APPLE_COLOR: Color = Color::srgb(0.8509804, 0.5294118, 0.3568627); // #d9875b

pub const CELL_SIZE: f32 = 50.0;
pub const GRID_SIZE: IVec2 = IVec2::new(10, 10);

pub const INIT_SNAKE_SEGMENT_COUNT: i32 = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, 1),
            Direction::Down => IVec2::new(0, -1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_key(key: &KeyCode) -> Option<Direction> {
        match key {
            KeyCode::ArrowUp => Some(Direction::Up),
            KeyCode::ArrowDown => Some(Direction::Down),
            KeyCode::ArrowLeft => Some(Direction::Left),
            KeyCode::ArrowRight => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Running,
    // Paused,
    GameOver,
}
