use crate::core::*;
use bevy::prelude::*;

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, initialize_apple);
        app.add_systems(Update, on_apple_consumed);
    }
}

/// Helper function to spawn an apple at a given position
fn spawn_apple(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: IVec2,
) {
    let shape = meshes.add(Circle::new(CELL_SIZE * 0.3));
    let material = MeshMaterial2d(materials.add(APPLE_COLOR));
    commands.spawn((
        Mesh2d(shape),
        material,
        Transform {
            translation: grid_pos_to_world_pos(position),
            ..default()
        },
        Apple,
        GridPosition(position),
    ));
}

fn initialize_apple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_position_query: Query<&GridPosition>,
    mut game_start_message_reader: MessageReader<GameStartMessage>,
) {
    if game_start_message_reader.is_empty() {
        return;
    }
    game_start_message_reader.clear();

    let apple_position = get_random_unoccupied_grid_pos(&grid_position_query);
    spawn_apple(&mut commands, &mut meshes, &mut materials, apple_position);
}

fn on_apple_consumed(
    mut commands: Commands,
    position_query: Query<&GridPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut apple_consumed_message_reader: MessageReader<AppleConsumedMessage>,
) {
    for message in apple_consumed_message_reader.read() {
        commands.entity(message.apple_entity).despawn();
        println!("Apple consumed!");

        let apple_position = get_random_unoccupied_grid_pos(&position_query);
        spawn_apple(&mut commands, &mut meshes, &mut materials, apple_position);
    }
}
