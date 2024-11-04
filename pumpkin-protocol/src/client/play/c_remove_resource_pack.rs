use pumpkin_macros::client_packet;
use serde::Serialize;
use uuid::Uuid;



#[derive(Serialize)]
#[client_packet("play:resource_pack_pop")]
pub struct CRemoveResourcePack {
    uuid: Option<Uuid>
}

impl CRemoveResourcePack {
    pub fn new(uuid: Option<Uuid>) -> Self {
        Self { uuid }
    }
}