use std::sync::Arc;

use crate::entity::player::Player;
use crate::entity::projectile::ThrownItemEntity;
use crate::item::pumpkin_item::PumpkinItem;
use crate::server::Server;
use async_trait::async_trait;
use pumpkin_data::entity::EntityType;
use pumpkin_data::sound::Sound;
use pumpkin_macros::pumpkin_item;
use pumpkin_world::item::registry::Item;
#[pumpkin_item("minecraft:snowball")]
pub struct SnowBallItem;

const POWER: f32 = 1.5;

#[async_trait]
impl PumpkinItem for SnowBallItem {
    async fn normal_use(&self, _block: &Item, player: &Player, server: &Server) {
        let position = player.position();
        let world = player.world();
        world
            .play_sound(
                Sound::EntitySnowballThrow,
                pumpkin_data::sound::SoundCategory::Neutral,
                &position,
            )
            .await;
        let entity = server.add_entity(position, EntityType::Snowball, world);

        let snowball = ThrownItemEntity::new(entity, &player.living_entity.entity);
        snowball.set_velocity_shooter_rot(&player.living_entity.entity, POWER, 1.0);

        world.spawn_entity(Arc::new(snowball)).await;
    }
}
