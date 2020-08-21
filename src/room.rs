use crate::player::Player;
use std::collections::HashMap;
use std::borrow::Borrow;

pub struct Room {
    room_id: u16,
    player: HashMap<u32, Player>
}

impl Room {
    pub fn new(room_id: u16) -> Self {
        Room {
            room_id,
            player: HashMap::new()
        }
    }
    pub fn room_id(&self) -> &u16 {
        &self.room_id
    }
    pub fn add_player(&mut self, id: u32, player: Player) {
        &self.player.insert(id, player);
    }
    pub fn get_player(&self, id: u32) -> Option<&Player> {
        self.player.get(&id)
    }
}