use std::collections::HashMap;

use uuid::Uuid;


pub struct Resourcepack {
    pub url: String,
    /// 40 character hex string with SHA-1 hash
    /// If it's not a 40 character hex string, the client will not use it for hash verification and likely waste bandwidth. 
    pub hash: Option<String>,
    pub forced: bool,
    pub prompt: Option<String>
}

pub type Resourcepacks = HashMap<Uuid, Resourcepack>;

impl Resourcepack {
    pub fn new(
        url: String,
        hash: Option<String>,
        forced: bool,
        prompt: Option<String>
    ) -> Self {
        Self {
            url,
            hash,
            forced,
            prompt
        }
    }
}
