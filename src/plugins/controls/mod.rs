use crate::core::*;
use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_direction);
    }
}

pub fn update_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_head_direction: ResMut<SnakeHeadDirection>,
) {
    for key in keyboard_input.get_just_pressed() {
        if let Some(new_direction) = Direction::from_key(key) {
            // prevent reversing direction
            if new_direction != snake_head_direction.direction.opposite() {
                snake_head_direction.direction = new_direction;
            }
        }
    }
}
