use bevy::prelude::*;

#[derive(Message, Default)]
pub struct PropelSnakeMessage;

#[derive(Message, Default)]
pub struct CollisionDetectedMessage;

#[derive(Message)]
pub struct AppleConsumedMessage {
    pub apple_entity: Entity,
}
