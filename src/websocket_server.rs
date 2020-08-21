extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::{Message, OwnedMessage};
// use crate::room::Room;
use crate::player::Player;
use std::sync::{Mutex, Arc};
use crate::room::Room;

pub fn run() {
    let server = Server::bind("0.0.0.0:9766").unwrap();
    let mut room = Arc::new(Mutex::new(Room::new(115)));
    for connection in server.filter_map(Result::ok) {
        println!("有一个用户连接了");
        let room = room.clone();
        thread::spawn(move || {
            let client = connection.accept().unwrap();
            let mut room = room.lock().unwrap();
            room.add_player(222, Player {
                id: 222,
                name: "我日日".to_string(),
                connection: &client,
            });

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("{:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                };

                match message {
                    OwnedMessage::Text(txt) => {
                        // sender.send_message(&OwnedMessage::Ping(b"PING".to_vec())).unwrap()
                        sender.send_message(&OwnedMessage::Text(txt)).unwrap()
                    }
                    OwnedMessage::Binary(bin) => {
                        sender.send_message(&OwnedMessage::Binary(bin)).unwrap()
                    }
                    OwnedMessage::Close(_) => {
                        sender.send_message(&OwnedMessage::Close(None)).ok();
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        sender.send_message(&OwnedMessage::Pong(data)).unwrap();
                    }
                    _ => (),
                }
            }
        });
    }

    println!("是否是等待过程");
}