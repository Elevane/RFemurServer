use crate::{
    game::{
        game_state::GameState,
        player::{self, Player},
    },
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
        let game_state = self.game_state.lock().unwrap();
        if packet.operation as i8 != 0 {
            {
                let mut identity = Identity::authenticate(packet.token);
                if identity.clone().is_none() {
                    return println!("Unauthorized");
                }
                let players_lock = game_state
                    .players
                    .lock()
                    .expect("Could not lock gamestate lock");
                let found_player = players_lock
                    .iter()
                    .find(|p| p.uid == identity.as_mut().unwrap().uid)
                    .cloned();

                match found_player {
                    Some(p) => {
                        player = p;
                    }
                    None => {
                        panic!("player isn't in game state")
                    }
                }
            }
        } else {
            player = Player::new(tcp_stream);
            game_state.add_player(player.clone());
        }
        if let Some(handler) = self.handlers.get(&(packet.operation as i8)) {
            handler.handle(game_state.clone(), Some(&packet.content), player);
        } else {
            println!("No handler for operation");
        }
    }

    pub fn remove_connection(&self, stream: &TcpStream) {
        let mut player_to_remove = None;
        {
            let game_state = self.game_state.lock().unwrap();
            let players = game_state.players.lock().unwrap();
            player_to_remove = players
                .iter()
                .find(|p| p.get_peer_addr() == stream.peer_addr().unwrap())
                .cloned();
        }
        if let Some(player) = player_to_remove {
            let mut game_state = self.game_state.lock().unwrap();
            game_state.remove_player(player);
        }
    }
}
