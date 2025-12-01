use bevy::prelude::*;

#[derive(Message, Default)]
pub struct PropelSnakeMessage;

#[derive(Message, Default)]
pub struct CollisionDetectedMessage;

#[derive(Message)]
pub struct AppleConsumedMessage {
    pub apple_entity: Entity,
}

#[derive(Message, Default)]
pub struct GameStartMessage;

#[derive(Message, Default)]
pub struct GameResetMessage;
