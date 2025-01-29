use pumpkin_macros::{cancellable, Event};
use pumpkin_util::text::TextComponent;
use std::sync::Arc;

use crate::entity::player::Player;

pub trait PlayerEvent: Send + Sync {
    fn get_player(&self) -> &Arc<Player>;
}

#[cancellable]
#[derive(Event, Clone)]
pub struct PlayerJoinEvent {
    pub player: Arc<Player>,
    pub join_message: TextComponent,
}

impl PlayerJoinEvent {
    pub fn new(player: Arc<Player>, join_message: TextComponent) -> Self {
        Self {
            player,
            join_message,
            cancelled: false,
        }
    }
}

impl PlayerEvent for PlayerJoinEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}

#[cancellable]
#[derive(Event, Clone)]
pub struct PlayerLeaveEvent {
    pub player: Arc<Player>,
    pub leave_message: TextComponent,
}

impl PlayerLeaveEvent {
    pub fn new(player: Arc<Player>, leave_message: TextComponent) -> Self {
        Self {
            player,
            leave_message,
            cancelled: false,
        }
    }
}

impl PlayerEvent for PlayerLeaveEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}
