use serde_json::{Value};
use std::sync::Arc;
use crate::room::Room;
use crate::player::Player;
use uuid::Uuid;
use crate::proto::{SendMessage, SendParcel};
use crate::recv_proto::{Controller, AddRoomJson, JoinRoomJson};

pub async fn recv_message(data: &str, room: Arc<Room>, client_id: Uuid) {
    let v: Value = serde_json::from_str(data).unwrap();
    match Controller::from_u8(v["mid"].as_u64().unwrap() as u8) {
        Controller::AddRoom => {
            add_room(data, room, client_id).await;
        }
        Controller::JoinRoom => {
            join_room(data, room, client_id).await;
        }
        Controller::SendMessage => {
            send_message(data, room, client_id).await;
        }
        _ => {
            println!("{}", data);
            println!("没有关联");
        }
    }
    // println!("{}", v["mid"]);
}

async fn add_room(data: &str, room: Arc<Room>, client_id: Uuid) {
    let json: AddRoomJson = serde_json::from_str(data).unwrap();
    let name = json.name;
    println!("{} 加入房间", name);
    room.add_player(client_id, Player {
        id: 123,
        client_id,
        name: name.clone(),
    }).await;
    room.send_all(SendParcel::SendMessage(SendMessage::new(name + ": 加入房间"))).await;
    // room.map_player(move |r, player| {
    //     r.send_message(player.client_id, SendParcel::SendMessage(SendMessage::new(name.clone() + ": 加入房间")))
    // }).await;
    // room.send_message();
    // println!("房间号：{}", room.get_player_name(123).await);
}

async fn join_room(data: &str, room: Arc<Room>, client_id: Uuid) {
    let json: JoinRoomJson = serde_json::from_str(data).unwrap();
    let name = json.name;
    println!("{} 加入房间", name);
    room.add_player(client_id, Player {
        id: 123,
        client_id,
        name: name.clone(),
    }).await;
}

async fn send_message(data: &str, room: Arc<Room>, client_id: Uuid) {
    let json: SendMessageJson = serde_json::from_str(data).unwrap();
    let message = json.message;
    for player in room.player.read().await.values() {
        if player.client_id != client_id {
            room.send_message(player.client_id, SendParcel::SendMessage(SendMessage::new(message.clone())))
        }
    }
}

pub async fn recv_binary(data: Vec<u8>, room: Arc<Room>, client_id: Uuid) {
    println!("{:?}", data);
    room.send_message(client_id, SendParcel::SendKeyboard(data));
}
