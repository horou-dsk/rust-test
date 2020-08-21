use std::collections::HashMap;
use crate::player::Player;
use tokio::sync::{RwLock, broadcast};
use crate::proto::{SendWrap, SendParcel, SendMessage};
use uuid::Uuid;

const MAX_CHANNEL: usize = 65536;

pub struct Room {
    room_id: u16,
    pub player: RwLock<HashMap<Uuid, Player>>,
    output_sender: broadcast::Sender<SendWrap>,
}

impl Room {
    pub fn new(room_id: u16) -> Self {
        let (output_sender, _) = broadcast::channel(MAX_CHANNEL);
        Room {
            room_id,
            player: Default::default(),
            output_sender,
        }
    }
    pub fn room_id(&self) -> &u16 {
        &self.room_id
    }
    pub async fn add_player(&self, id: Uuid, player: Player) {
        self.player.write().await.entry(id).or_insert(player);
    }
    pub async fn get_player_name(&self, id: Uuid) -> String {
        self.player.read().await.get(&id).unwrap().name.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SendWrap> {
        self.output_sender.subscribe()
    }

    pub fn send_message(&self, client_id: Uuid, message: SendParcel) {
        self.output_sender.send(SendWrap::new(client_id, message)).unwrap();
    }

    pub async fn send_all(&self, message: SendParcel) {
        for player in self.player.read().await.values() {
            let message = message.clone();
            self.output_sender.send(SendWrap::new(player.client_id, message)).unwrap();
        }
    }

    pub async fn disconnect(&self, client_id: Uuid) {
        let player = self.player.write().await.remove(&client_id).unwrap();
        let name = player.name;
        self.send_message(client_id, SendParcel::Disconnect);
        self.send_all(SendParcel::SendMessage(SendMessage::new(name + "断开连接！"))).await;
    }

    // pub async fn map_player<F>(&self, f: F)
    //     where
    //         F: FnOnce(Player),
    //         F: Send + 'static,
    // {
    //     for player in self.player.read().await.values() {
    //         f(player.clone())
    //     }
    // }

    // pub async fn get_player(&self, id: u32) -> Option<Player> {
    //     self.player.read().await.get(&id)
    // }
}