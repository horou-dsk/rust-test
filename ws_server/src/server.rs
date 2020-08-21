use log::*;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error, tungstenite::Message};
use crate::room::Room;
use crate::controller::{recv_message, recv_binary};
use crate::client::Client;
use crate::proto::SendParcel;
use threadpool::ThreadPool;

async fn accept_connection(peer: SocketAddr, stream: TcpStream, room: Arc<Room>) {
    let client = Client::new();
    let client_id = client.id;
    if let Err(e) = handle_connection(peer, stream, room.clone(), client).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => {
                room.disconnect(client_id).await;
                error!("Error processing connection: {}", err)
            },
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream, room: Arc<Room>, client: Client) -> Result<(), Error> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    let (mut write, mut read) = ws_stream.split();
    let mut rx = room.subscribe();
    let client_id = client.id;
    tokio::spawn(async move {
        // client.write_output(rx.into_stream())
        //     .try_for_each(|message| async {
        //         write.send(message).await.unwrap();
        //         Ok(())
        //     })
        println!("开始通知");
        while let m = rx.next().await.unwrap().unwrap() {
            if m.client_id == client_id {
                let msg = match m.message {
                    SendParcel::Disconnect => break,
                    _ => m.message
                };
                println!("\n");
                println!("{:?}", msg);
                let data: String = serde_json::to_string(&msg).unwrap();
                match write.send(Message::from(data)).await {
                    Err(err) => match err {
                        Error::ConnectionClosed => {
                            println!("连接断开了啊");
                            break
                        }
                        _ => {}
                    }
                    _ => {}
                }
            }
        }
        // 'a: loop {
        //     // let m = rx.recv().await.unwrap();
        //     for m in rx.recv().await {
        //
        //     }
        // }
        println!("结束了");
    });
    while let Some(msg) = read.next().await {
        let msg = msg?;
        let client_id = client.id;
        if msg.is_text() {
            recv_message(msg.to_text().unwrap(), room.clone(), client_id).await;
            // write.send(msg).await?;
        } else if msg.is_binary() {
            recv_binary(msg.into_data(), room.clone(), client_id).await;
        }
    }

    room.disconnect(client_id).await;

    Ok(())
}

pub struct Server {
    port: u16,
    room: Arc<Room>
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            port,
            room: Arc::new(Room::new(10)),
        }
    }

    pub async fn run(&self) {
        let port = self.port;
        let room = self.room.clone();
        let addr = "0.0.0.0:".to_owned() + port.to_string().as_str();
        let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");
        info!("Listening on: {}", addr);
        // let pool = ThreadPool::new(50);
        let runtime = tokio::runtime::Builder::new()
            .threaded_scheduler()
            .core_threads(64)
            .build()
            .unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            info!("Peer address: {}", peer);

            runtime.spawn(accept_connection(peer, stream, room.clone()));
        }
    }
}
