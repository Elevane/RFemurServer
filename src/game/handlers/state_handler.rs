use crate::{
    game::game_state::GameState,
    tcp::{
        server::{auth::identity::Identity, packet::Packet},
        server_operation::ServerOperation,
    },
};

use std::{collections::HashMap, io::Write, net::TcpStream};

use super::{connect_handler::ConnectHandler, handler::Handler};
pub struct StateHandler {
    game_state: GameState,
    handlers: HashMap<i8, Box<dyn Handler>>,
}
impl StateHandler {
    pub(crate) fn init(game_state: GameState) -> StateHandler {
        let mut handlers = HashMap::<i8, Box<dyn Handler>>::new();
        handlers.insert(
            ServerOperation::ConnectServerRequest as i8,
            Box::new(ConnectHandler),
        );
        Self {
            game_state,
            handlers,
        }
    }

    pub fn handle(&self, packet: Packet, tcp_stream: TcpStream) {
        if packet.operation as i8 != 0 {
            let i = Identity::authenticate(packet.token);
            if i.is_none() {
                let _ = tcp_stream
                    .try_clone()
                    .unwrap()
                    .write(Packet::incorrect().as_bytes());
            }
        }
        if let Some(handler) = self.handlers.get(&(packet.operation as i8)) {
            handler.handle(&self.game_state, &packet.content, tcp_stream);
        } else {
            println!("No handler for operation");
        }
    }
}
