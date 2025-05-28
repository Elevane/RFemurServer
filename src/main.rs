mod game;
mod tcp;
use tcp::server::game_server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new(Some(5000), Some(3333));
    let _ = server.start().await;
}
