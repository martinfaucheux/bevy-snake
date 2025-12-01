use crate::core::*;
use bevy::prelude::*;

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_apple);
        app.add_systems(Update, on_apple_consumed);
    }
}

fn create_apple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_position_query: Query<&GridPosition>,
) {
    let apple_position = get_random_unoccupied_grid_pos(&grid_position_query);

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

fn on_apple_consumed(
    mut commands: Commands,
    mut apple_consumed_message_reader: MessageReader<AppleConsumedMessage>,
) {
    for message in apple_consumed_message_reader.read() {
        commands.entity(message.apple_entity).despawn();
        println!("Apple consumed!");
    }
}
