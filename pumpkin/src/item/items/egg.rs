use std::sync::Arc;

use crate::entity::player::Player;
use crate::entity::projectile::ThrownItem;
use crate::item::pumpkin_item::PumpkinItem;
use crate::server::Server;
use async_trait::async_trait;
use pumpkin_data::entity::EntityType;
use pumpkin_data::sound::{Sound, SoundCategory};
use pumpkin_macros::pumpkin_item;
use pumpkin_world::item::registry::Item;

use super::THROW_POWER;

#[pumpkin_item("minecraft:egg")]
pub struct EggItem;

#[async_trait]
impl PumpkinItem for EggItem {
    async fn normal_use(&self, _block: &Item, player: &Player, server: &Server) {
        let position = player.position();
        let world = player.world();
        world
            .play_sound(Sound::EntityEggThrow, SoundCategory::Players, position)
            .await;
        // TODO: Implement eggs the right way, so there is a chance of spawning chickens
        let entity = server.add_entity(position, EntityType::Egg, world);
        let snowball = ThrownItem::new(entity, &player.living_entity.entity);
        snowball.set_velocity_shooter_rot(&player.living_entity.entity, THROW_POWER, 1.0);
        world.spawn_entity(Arc::new(snowball)).await;
    }
}
