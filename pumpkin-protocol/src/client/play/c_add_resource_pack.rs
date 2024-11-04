use pumpkin_core::text::TextComponent;
use pumpkin_macros::client_packet;
use serde::Serialize;
use uuid::Uuid;



#[derive(Serialize)]
#[client_packet("play:resource_pack_push")]
pub struct CAddResourcePack<'a> {
    uuid: Uuid,
    url: String,
    /// 40 character hex string with SHA-1 hash
    /// If it's not a 40 character hex string, the client will not use it for hash verification and likely waste bandwidth. 
    hash: String,
    forced: bool,
    prompt_message: Option<TextComponent<'a>>
}

impl<'a> CAddResourcePack<'a> {
    pub fn new(
        uuid: Uuid,
        url: String,
        hash: Option<String>,
        forced: bool,
        prompt_message: Option<TextComponent<'a>>
    ) -> Self {
        Self {
            uuid,
            url,
            hash: match hash {
                None => String::new(),
                Some(hash) => hash
            },
            forced,
            prompt_message
        }
    }
}