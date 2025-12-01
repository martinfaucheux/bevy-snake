use crate::core::*;
use bevy::prelude::*;

use rand::Rng;

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_apple);
    }
}

fn create_apple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_position_query: Query<&GridPosition>,
) {
    let mut apple_position = get_random_grid_pos();
    loop {
        if !grid_position_query
            .iter()
            .any(|pos| pos.0 == apple_position)
        {
            break;
        }
        apple_position = get_random_grid_pos();
    }

    let shape = meshes.add(Circle::new(CELL_SIZE * 0.3));
    let material = MeshMaterial2d(materials.add(APPLE_COLOR));
    commands.spawn((
        Mesh2d(shape),
        material,
        Transform {
            translation: grid_pos_to_world_pos(apple_position),
            ..default()
        },
        Apple,
        GridPosition(apple_position),
    ));
}

fn get_random_grid_pos() -> IVec2 {
    IVec2::new(
        rand::rng().random_range(0..GRID_SIZE.x) as i32,
        rand::rng().random_range(0..GRID_SIZE.y) as i32,
    )
}
