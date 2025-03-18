use crate::{
    game::{game_state::GameState, player::Player},
    tcp::{
        server::{auth::identity::Identity, packet::Packet},
        server_operation::ServerOperation,
    },
};

use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use super::{connect_handler::ConnectHandler, handler::Handler, move_handler::MoveHandler};
pub struct StateHandler {
    game_state: Arc<Mutex<GameState>>,
    handlers: HashMap<i8, Box<dyn Handler>>,
}
impl StateHandler {
    pub(crate) fn init(game_state: Arc<Mutex<GameState>>) -> StateHandler {
        let mut handlers = HashMap::<i8, Box<dyn Handler>>::new();
        handlers.insert(
            ServerOperation::ConnectServerRequest as i8,
            Box::new(ConnectHandler),
        );
        handlers.insert(ServerOperation::MoveRequest as i8, Box::new(MoveHandler));
        Self {
            game_state,
            handlers,
        }
    }

    pub fn handle(&self, packet: Packet, tcp_stream: TcpStream) {
        let player: Player;
        if packet.operation as i8 != 0 {
            let mut identity = Identity::authenticate(packet.token);
            if identity.clone().is_none() {
                let _ = tcp_stream
                    .try_clone()
                    .unwrap()
                    .write(Packet::incorrect().as_bytes());
            }
            // Parcourir le vecteur pour chercher un Player avec le uid correspondant
            {
                let game_state = self.game_state.lock().unwrap();

                // Verrouille ensuite la liste des joueurs
                let players_lock = game_state.players.lock().unwrap();
                player = players_lock
                    .iter()
                    .find(|p| p.uid == identity.as_mut().unwrap().uid)
                    .cloned()
                    .unwrap();
            }
        } else {
            player = Player::new(tcp_stream);
        }
        if let Some(handler) = self.handlers.get(&(packet.operation as i8)) {
            handler.handle(&self.game_state, &packet.content, player);
        } else {
            println!("No handler for operation");
        }
    }
}
