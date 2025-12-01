use crate::core::*;
use bevy::prelude::*;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // add messages
        app.add_message::<PropelSnakeMessage>();
        app.add_message::<SegmentMovedMessage>();
        // insert all resources
        app.insert_resource(SnakeHeadDirection::default());
        app.insert_resource(GameTickTimer::default());
    }
}
