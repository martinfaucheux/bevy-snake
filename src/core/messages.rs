use bevy::prelude::*;

#[derive(Message, Default)]
pub struct PropelSnakeMessage;

#[derive(Message, Default)]
pub struct SegmentMovedMessage {
    pub segment_index: usize,
    pub from_position: IVec2,
    pub to_position: IVec2,
}
