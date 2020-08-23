use serde::{Deserialize, Serialize};

pub enum Controller {
    AddRoom,
    JoinRoom,
    QuitRoom,
    SendMessage,
    GameStatus,
    NotFound,
}

impl Controller {
    pub fn from_u8(value: u8) -> Controller {
        match value {
            1 => Controller::AddRoom,
            2 => Controller::JoinRoom,
            3 => Controller::QuitRoom,
            4 => Controller::SendMessage,
            5 => Controller::GameStatus,
            _ => Controller::NotFound,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddRoomJson {
    mid: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomJson {
    mid: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageJson {
    mid: u8,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameStatusJson {
    mid: u8,
    pub json_data: String,
}