use bevy::prelude::*;

#[derive(Message, Default)]
pub struct PropelSnakeMessage;

#[derive(Message, Default)]
pub struct CollisionDetectedMessage;
