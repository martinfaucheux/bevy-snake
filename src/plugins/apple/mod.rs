use crate::core::*;
use bevy::prelude::*;

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
) {
    let shape = meshes.add(Circle::new(CELL_SIZE * 0.3));
    let material = MeshMaterial2d(materials.add(APPLE_COLOR));
    let apple_grid_pos = IVec2::new(1, 1);
    commands.spawn((
        Mesh2d(shape),
        material,
        Transform {
            translation: grid_pos_to_world_pos(apple_grid_pos),
            ..default()
        },
        Apple,
        GridPosition(apple_grid_pos),
    ));
}
