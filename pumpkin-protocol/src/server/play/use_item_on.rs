use pumpkin_data::packet::serverbound::PLAY_USE_ITEM_ON;
use pumpkin_macros::server_packet;
use pumpkin_util::math::{position::BlockPos, vector3::Vec3};
use serde::Deserialize;

use crate::VarInt;

#[derive(Deserialize)]
#[server_packet(PLAY_USE_ITEM_ON)]
pub struct SUseItemOn {
    pub hand: VarInt,
    pub location: BlockPos,
    pub face: VarInt,
    pub cursor_pos: Vec3<f32>,
    pub inside_block: bool,
    pub is_against_world_border: bool,
    pub sequence: VarInt,
}
