mod game;
mod tcp;
use tcp::server::game_server::Server;
fn main() {
    let mut server = Server::new(None, None);
    let _ = server.start();
}
