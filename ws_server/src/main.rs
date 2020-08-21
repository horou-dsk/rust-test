use ws_server::server::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Server::new(9766);
    server.run().await;
}
