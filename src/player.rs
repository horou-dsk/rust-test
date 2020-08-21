use websocket::server::upgrade::WsUpgrade;
use websocket::server::upgrade::sync::Buffer;
use std::net::TcpStream;
use websocket::sync::Client;

pub struct Player {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) connection: &'static Client<TcpStream>,
}