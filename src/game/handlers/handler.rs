use std::net::TcpStream;

use crate::game::game_state::GameState;

// Trait pour définir un Handler
pub trait Handler {
    fn handle(&self, game_state: &GameState, data: &str, tcp_stream: TcpStream);
}
