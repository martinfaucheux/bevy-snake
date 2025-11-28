use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const CELL_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);

const CELL_BORDER_THICKNESS: f32 = 2.0;

const BALL_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

const CELL_SIZE: f32 = 50.0;
const GRID_SIZE: IVec2 = IVec2::new(10, 10);

const TICK_DURATION: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Snake".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameTickTimer(Timer::from_seconds(
            TICK_DURATION,
            TimerMode::Repeating,
        )))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_direction, move_snake))
        .run();
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct Direction(IVec2);

#[derive(Component)]
struct GridPosition(IVec2);

#[derive(Resource)]
struct GameTickTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    let shape = meshes.add(Circle::new((CELL_SIZE / 2.0) * 0.8));
    let init_grid_pos = world_pos_to_grid_pos(Vec3::ZERO);
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform {
            translation: grid_pos_to_world_pos(init_grid_pos),
            ..default()
        },
        SnakeHead,
        Direction(IVec2::X),
        GridPosition(init_grid_pos),
    ));
}

fn update_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ball_direction: Single<&mut Direction, With<SnakeHead>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        ball_direction.0 = IVec2::NEG_X;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        ball_direction.0 = IVec2::X;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        ball_direction.0 = IVec2::Y;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        ball_direction.0 = IVec2::NEG_Y;
    }
}

fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<GameTickTimer>,
    snake_query: Single<(&mut Transform, &mut GridPosition, &Direction), With<SnakeHead>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut snake_transform, mut grid_position, snake_direction) = snake_query.into_inner();

        grid_position.0 += snake_direction.0;
        grid_position.0 %= IVec2::new(GRID_SIZE.x as i32, GRID_SIZE.y as i32);

        snake_transform.translation = grid_pos_to_world_pos(grid_position.0);
    }
}

fn grid_pos_to_world_pos(grid_pos: IVec2) -> Vec3 {
    Vec3::new(
        grid_pos.x as f32 * CELL_SIZE - (GRID_SIZE.x as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
        grid_pos.y as f32 * CELL_SIZE - (GRID_SIZE.y as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
        0.0,
    )
}

fn world_pos_to_grid_pos(world_pos: Vec3) -> IVec2 {
    IVec2::new(
        (((world_pos.x + (GRID_SIZE.x as f32 * CELL_SIZE) / 2.0) / CELL_SIZE).floor()) as i32,
        (((world_pos.y + (GRID_SIZE.y as f32 * CELL_SIZE) / 2.0) / CELL_SIZE).floor()) as i32,
    )
}
