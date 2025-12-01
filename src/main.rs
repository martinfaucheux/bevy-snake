use bevy::prelude::*;

mod core;
mod plugins;

use core::*;
use plugins::*;

use crate::plugins::ControlPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Snake".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            SharedPlugin,
            ControlPlugin,
            GamePlugin,
            SnakePlugin,
            ApplePlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
