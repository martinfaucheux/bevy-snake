use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const CIRCLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const CIRCLE_SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(CIRCLE_SIZE));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(CIRCLE_COLOR)),
        Transform::default(),
    ));
}
