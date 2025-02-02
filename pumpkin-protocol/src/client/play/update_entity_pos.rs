use pumpkin_data::packet::clientbound::PLAY_MOVE_ENTITY_POS;
use pumpkin_macros::client_packet;
use pumpkin_util::math::vector3::Vec3;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize)]
#[client_packet(PLAY_MOVE_ENTITY_POS)]
pub struct CUpdateEntityPos {
    entity_id: VarInt,
    delta: Vec3<i16>,
    on_ground: bool,
}

impl CUpdateEntityPos {
    pub fn new(entity_id: VarInt, delta: Vec3<i16>, on_ground: bool) -> Self {
        Self {
            entity_id,
            delta,
            on_ground,
        }
    }
}
