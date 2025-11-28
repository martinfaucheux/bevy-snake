use crate::core::*;
use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {}
}

// fn update_direction_on_key_press(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut direction: ResMut<Direction>,
// ) {
//     for key in keyboard_input.get_just_pressed() {
//         if let Some(new_direction) = Direction::from_key(key) {
//             *direction = new_direction;
//         }
//     }
// }
