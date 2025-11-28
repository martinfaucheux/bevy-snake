use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const BALL_SIZE: f32 = 50.0;
const BALL_SPEED: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_ball)
        .run();
}

#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(BALL_SIZE));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::default(),
        Ball,
    ));
}

fn move_ball(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ball_transform: Single<&mut Transform, With<Ball>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= Vec2::Y;
    }

    // Calculate the new horizontal ball position based on player input
    let new_ball_position =
        ball_transform.translation + direction.extend(0.0) * BALL_SPEED * time.delta_secs();

    ball_transform.translation = new_ball_position;
}
