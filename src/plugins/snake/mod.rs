use crate::core::*;
use bevy::prelude::*;
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_snake);
        app.add_systems(Update, move_snake);
    }
}

fn create_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        GridPosition(init_grid_pos),
    ));
}

fn move_snake(
    snake_query: Single<(&mut Transform, &mut GridPosition), With<SnakeHead>>,
    snake_head_direction: Res<SnakeHeadDirection>,
    mut propel_snake_message: MessageReader<PropelSnakeMessage>,
) {
    let (mut snake_transform, mut grid_position) = snake_query.into_inner();
    for _ in propel_snake_message.read() {
        grid_position.0 += snake_head_direction.direction.to_ivec2();

        // euclid remaining
        grid_position.0 = grid_position.0.rem_euclid(GRID_SIZE);

        snake_transform.translation = grid_pos_to_world_pos(grid_position.0);
    }
}
