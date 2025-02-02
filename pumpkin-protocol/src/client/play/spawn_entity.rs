use pumpkin_data::packet::clientbound::PLAY_ADD_ENTITY;
use pumpkin_macros::client_packet;
use pumpkin_util::math::vector3::Vec3;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize)]
#[client_packet(PLAY_ADD_ENTITY)]
pub struct CSpawnEntity {
    entity_id: VarInt,
    #[serde(with = "uuid::serde::compact")]
    entity_uuid: uuid::Uuid,
    typ: VarInt,
    position: Vec3<f64>,
    pitch: u8,    // angle
    yaw: u8,      // angle
    head_yaw: u8, // angle
    data: VarInt,
    velocity: Vec3<i16>,
}

impl CSpawnEntity {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        entity_id: VarInt,
        entity_uuid: uuid::Uuid,
        typ: VarInt,
        position: Vec3<f64>,
        pitch: f32,    // angle
        yaw: f32,      // angle
        head_yaw: f32, // angle
        data: VarInt,
        velocity: Vec3<f32>,
    ) -> Self {
        Self {
            entity_id,
            entity_uuid,
            typ,
            position,
            pitch: (pitch * 256.0 / 360.0).floor() as u8,
            yaw: (yaw * 256.0 / 360.0).floor() as u8,
            head_yaw: (head_yaw * 256.0 / 360.0).floor() as u8,
            data,
            velocity: Vec3::new(
                (velocity.x.clamp(-3.9, 3.9) * 8000.0) as i16,
                (velocity.y.clamp(-3.9, 3.9) * 8000.0) as i16,
                (velocity.z.clamp(-3.9, 3.9) * 8000.0) as i16,
            ),
        }
    }
}
