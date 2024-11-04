use pumpkin_core::{math::position::WorldPosition, GameMode};
use pumpkin_macros::client_packet;
use serde::Serialize;

use crate::{Identifier, VarInt};


/// Respawn packet
/// 
/// Warning:
/// Avoid changing player's dimension to same dimension they were already in unless they are dead.
/// If you change the dimension to one they are already in, weird bugs can occur,
/// such as the player being unable to attack other players in new world (until they die and respawn).
/// Before 1.16, if you must respawn a player in the same dimension without killing them, send two respawn packets,
/// one to a different world and then another to the world you want. You do not need to complete the first respawn;
/// it only matters that you send two packets.
#[derive(Serialize)]
#[client_packet("play:respawn")]
pub struct CRespawn {
    dimension_type: VarInt,
    dimension_name: Identifier,
    hashed_seed: i64,
    game_mode: GameMode,
    prev_game_mode: GameMode,
    is_debug: bool,
    is_flat: bool,
    death_location: Option<DeathLocation>,
    portal_cooldown: VarInt,
    data_kept: u8
}

#[derive(Serialize)]
pub struct DeathLocation {
    death_dimension: Identifier,
    death_pos: WorldPosition
}

impl CRespawn {
    pub fn new(
        dimension_type: VarInt,
        dimension_name: Identifier,
        hashed_seed: i64,
        game_mode: GameMode,
        prev_game_mode: GameMode,
        is_debug: bool,
        is_flat: bool,
        death_location: Option<DeathLocation>,
        portal_cooldown: VarInt,
        data_kept: u8
    ) -> Self {
        Self {
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode: match game_mode {
                GameMode::Undefined => panic!("Undefined gamemode is not allowed in CRespawn packet (not prev gamemode)"),
                other => other
            },
            prev_game_mode,
            is_debug,
            is_flat,
            death_location,
            portal_cooldown,
            data_kept
        }
    }
}