use crate::core::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid);
        app.add_systems(
            FixedUpdate,
            game_time.run_if(on_timer(Duration::from_millis(
                (TICK_DURATION * 1000.0) as u64,
            ))),
        );
    }
}

fn game_time(mut propel_snake_event: MessageWriter<PropelSnakeMessage>) {
    propel_snake_event.write(PropelSnakeMessage);
    println!("Game Tick");
}

fn setup_grid(mut commands: Commands) {
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
