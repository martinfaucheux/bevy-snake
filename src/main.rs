use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const CELL_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);

const CELL_BORDER_THICKNESS: f32 = 2.0;

const BALL_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

const CELL_SIZE: f32 = 50.0;
const GRID_SIZE: Vec2 = Vec2::new(10.0, 10.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct SnakeHead;

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
                        col as f32 * CELL_SIZE - (GRID_SIZE.x * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
                        row as f32 * CELL_SIZE - (GRID_SIZE.y * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
                        // in background
                        -10.0,
                    ),
                    ..default()
                },
            ));
        }
    }

    let shape = meshes.add(Circle::new((CELL_SIZE / 2.0) * 0.8));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform {
            translation: grid_pos_to_world_pos(Vec2::new(GRID_SIZE.x / 2.0, GRID_SIZE.y / 2.0)),
            ..default()
        },
        SnakeHead,
    ));
}

// fn move_ball(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut ball_transform: Single<&mut Transform, With<SnakeHead>>,
//     time: Res<Time>,
// ) {
//     let mut direction = Vec2::ZERO;

//     if keyboard_input.pressed(KeyCode::ArrowLeft) {
//         direction -= Vec2::X;
//     }

//     if keyboard_input.pressed(KeyCode::ArrowRight) {
//         direction += Vec2::X;
//     }

//     if keyboard_input.pressed(KeyCode::ArrowUp) {
//         direction += Vec2::Y;
//     }

//     if keyboard_input.pressed(KeyCode::ArrowDown) {
//         direction -= Vec2::Y;
//     }

//     // Calculate the new horizontal ball position based on player input
//     let new_ball_position =
//         ball_transform.translation + direction.extend(0.0) * BALL_SPEED * time.delta_secs();

//     ball_transform.translation = new_ball_position;
// }

fn grid_pos_to_world_pos(grid_pos: Vec2) -> Vec3 {
    Vec3::new(
        grid_pos.x * CELL_SIZE - (GRID_SIZE.x * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
        grid_pos.y * CELL_SIZE - (GRID_SIZE.y * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
        0.0,
    )
}

fn world_pos_to_grid_pos(world_pos: Vec3) -> Vec2 {
    Vec2::new(
        ((world_pos.x + (GRID_SIZE.x * CELL_SIZE) / 2.0) / CELL_SIZE).floor(),
        ((world_pos.y + (GRID_SIZE.y * CELL_SIZE) / 2.0) / CELL_SIZE).floor(),
    )
}
