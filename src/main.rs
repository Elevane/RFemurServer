mod game;
mod tcp;
use tcp::server::game_server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new(Some(5000), Some(3333));
    let address = format!("{}:{}", "127.0.0.1", 33333);
    println!("Starting server on {}", address);
    server.start().await;
}
