use crate::core::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            game_time.run_if(on_timer(Duration::from_millis(
                (TICK_DURATION * 1000.0) as u64,
            ))),
        );
    }
}

fn game_time(mut propel_snake_event: MessageWriter<PropelSnakeMessage>) {
    propel_snake_event.write(PropelSnakeMessage);
    println!("Game Tick");
}
