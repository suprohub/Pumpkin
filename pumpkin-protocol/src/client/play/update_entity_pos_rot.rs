use pumpkin_data::packet::clientbound::PLAY_MOVE_ENTITY_POS_ROT;
use pumpkin_macros::client_packet;
use pumpkin_util::math::vector3::Vec3;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize)]
#[client_packet(PLAY_MOVE_ENTITY_POS_ROT)]
pub struct CUpdateEntityPosRot {
    entity_id: VarInt,
    delta: Vec3<i16>,
    yaw: u8,
    pitch: u8,
    on_ground: bool,
}

impl CUpdateEntityPosRot {
    pub fn new(entity_id: VarInt, delta: Vec3<i16>, yaw: u8, pitch: u8, on_ground: bool) -> Self {
        Self {
            entity_id,
            delta,
            yaw,
            pitch,
            on_ground,
        }
    }
}
