use crate::world::World;
use pumpkin_macros::{cancellable, Event};
use pumpkin_world::chunk::ChunkData;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkLoad {
    pub world: Arc<World>,
    pub chunk: Arc<RwLock<ChunkData>>,
}

#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkSend {
    pub world: Arc<World>,
    pub chunk: Arc<RwLock<ChunkData>>,
}

#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkSave {
    pub world: Arc<World>,
    pub chunk: Arc<RwLock<ChunkData>>,
}
