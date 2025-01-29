use crate::world::World;
use pumpkin_macros::{cancellable, Event};
use pumpkin_protocol::client::play::CChunkData;
use pumpkin_world::chunk::ChunkData;
use std::sync::Arc;

#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkLoad {
    pub world: Arc<World>,
    pub chunk: ChunkData,
}

#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkSend {
    pub world: Arc<World>,
    pub chunk: ChunkData,
}

#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkSave {
    pub world: Arc<World>,
    pub chunk: ChunkData,
}
