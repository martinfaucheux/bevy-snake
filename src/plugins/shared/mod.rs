use crate::core::*;
use bevy::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // add messages
        app.add_message::<PropelSnakeMessage>();
        app.add_message::<CollisionDetectedMessage>();
        app.add_message::<AppleConsumedMessage>();
        app.add_message::<GameStartMessage>();
        app.add_message::<GameResetMessage>();
        // insert all resources
        app.insert_resource(SnakeHeadDirection::default());
        app.insert_resource(GameStateResource::default());
    }
}
