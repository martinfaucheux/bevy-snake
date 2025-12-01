use crate::core::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_game);
        app.add_systems(
            FixedUpdate,
            generate_game_ticks.run_if(on_timer(Duration::from_millis(
                (TICK_DURATION * 1000.0) as u64,
            ))),
        );
        app.add_systems(Update, (detect_game_over, reset_game));
    }
}

fn generate_game_ticks(
    mut propel_snake_event: MessageWriter<PropelSnakeMessage>,
    game_state: Res<GameStateResource>,
) {
    if let GameState::Running = game_state.0 {
        propel_snake_event.write(PropelSnakeMessage);
        println!("Game Tick");
    }
}

fn detect_game_over(
    game_over_message_reader: MessageReader<CollisionDetectedMessage>,
    mut game_state: ResMut<GameStateResource>,
) {
    if game_state.0 == GameState::GameOver || game_over_message_reader.is_empty() {
        return;
    }

    game_state.0 = GameState::GameOver;
    println!("Game Over!");
}

fn initialize_game(
    mut commands: Commands,
    mut game_start_message_writer: MessageWriter<GameStartMessage>,
) {
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
    game_start_message_writer.write(GameStartMessage);
}

fn reset_game(
    mut commands: Commands,
    mut q: ParamSet<(
        Query<Entity, With<SnakeHead>>,
        Query<Entity, With<SnakeSegment>>,
        Query<Entity, With<Apple>>,
    )>,
    mut game_state: ResMut<GameStateResource>,
    mut snake_head_direction: ResMut<SnakeHeadDirection>,
    mut game_reset_message_reader: MessageReader<GameResetMessage>,
    mut game_start_message_writer: MessageWriter<GameStartMessage>,
) {
    if game_reset_message_reader.is_empty() {
        return;
    }
    game_reset_message_reader.clear();

    println!("Reset Game 2");

    for entity in q.p0().iter_mut() {
        commands.entity(entity).despawn();
    }
    for entity in q.p1().iter_mut() {
        commands.entity(entity).despawn();
    }
    for entity in q.p2().iter_mut() {
        commands.entity(entity).despawn();
    }
    game_state.0 = GameState::Running;
    snake_head_direction.current_direction = Direction::Right;
    snake_head_direction.recorded_target_direction = Direction::Right;

    game_start_message_writer.write(GameStartMessage);
}
