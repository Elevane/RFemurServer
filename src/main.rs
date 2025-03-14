mod tcp;
use tcp::server::game_server::Server;
#[tokio::main]
async fn main() {
    let server = Server::new(None, None);
    server.start().await.unwrap();
}
