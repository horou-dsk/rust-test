use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SendParcel {
    JoinRoom(JoinRoomMsg),
    SendMessage(SendMessage),
    SendKeyboard(Vec<u8>),
    GetGameStatus(u8),
    Disconnect,
}

#[derive(Debug, Clone)]
pub struct SendWrap {
    pub client_id: Uuid,
    pub message: SendParcel,
}

impl SendWrap {
    pub fn new(client_id: Uuid, message: SendParcel) -> Self {
        SendWrap { client_id, message }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRoomMsg {
    room_id: u16,
    user_id: u32,
}

impl JoinRoomMsg {
    pub fn new(room_id: u16, user_id: u32) -> Self {
        JoinRoomMsg { room_id, user_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    message: String,
}

impl SendMessage {
    pub fn new(message: String) -> Self {
        SendMessage { message }
    }
}
