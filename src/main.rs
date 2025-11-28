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
        .add_plugins((SharedPlugin, ControlPlugin, GamePlugin, SnakePlugin))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // grid cells
    for row in 0..GRID_SIZE.y as i32 {
        for col in 0..GRID_SIZE.x as i32 {
            commands.spawn((
                Sprite {
                    color: CELL_COLOR,
                    custom_size: Some(Vec2::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(
                        col as f32 * CELL_SIZE - (GRID_SIZE.x as f32 * CELL_SIZE) / 2.0
                            + CELL_SIZE / 2.0,
                        row as f32 * CELL_SIZE - (GRID_SIZE.y as f32 * CELL_SIZE) / 2.0
                            + CELL_SIZE / 2.0,
                        // in background
                        -10.0,
                    ),
                    ..default()
                },
            ));
        }
    }
}
