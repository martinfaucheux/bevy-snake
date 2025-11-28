use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const SPRITE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(SPRITE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(100.0, 100.0, 0.0),
            ..default()
        },
    ));
}
