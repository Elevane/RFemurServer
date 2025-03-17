use std::net::TcpStream;

use crate::{game::game_state::GameState, tcp::{server::packet::Packet, server_operation::ServerOperation}};

use super::handler::{self, Handler};

pub struct MoveHandler;

impl Handler for MoveHandler {
    fn handle(&self, _game_state: &GameState, data: &str, tcp_stream: TcpStream) {
        println!("Handling move request with data: {}", data);
        let token = "{ x: 10, y: 20,  uid:}".to_string();
        let _ = tcp_stream
            .try_clone()
            .unwrap()
            .write(Packet::encode(ServerOperation::ConnectServerRequest, token).as_bytes());
    }
}
