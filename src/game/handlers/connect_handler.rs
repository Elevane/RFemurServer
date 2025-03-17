use std::{io::Write, net::TcpStream};

use crate::{
    game::game_state::GameState,
    tcp::{server::packet::Packet, server_operation::ServerOperation},
};

use super::handler::Handler;
pub struct ConnectHandler;

impl Handler for ConnectHandler {
    fn handle(&self, _game_state: &GameState, data: &str, tcp_stream: TcpStream) {
        println!("Handling connection request with data: {}", data);
        let token = "token-valid".to_string();
        let _ = tcp_stream
            .try_clone()
            .unwrap()
            .write(Packet::encode(ServerOperation::ConnectServerRequest, token).as_bytes());
    }
}
