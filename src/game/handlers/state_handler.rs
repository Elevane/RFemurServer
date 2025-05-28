use tokio::{net::TcpStream, sync::RwLock};

use crate::{
    game::{game_state::GameState, player::Player},
    tcp::{
        server::{auth::identity::Identity, packet::Packet},
        server_operation::ServerOperation,
    },
};

use std::{collections::HashMap, sync::Arc};

use super::{connect_handler::ConnectHandler, handler::Handler, move_handler::MoveHandler};

pub struct StateHandler {
    game_state: Arc<RwLock<GameState>>,
    handlers: HashMap<i8, Box<dyn Handler + Send + Sync>>,
}
impl StateHandler {
    pub(crate) fn init(game_state: Arc<RwLock<GameState>>) -> Self {
        let mut handlers = HashMap::<i8, Box<dyn Handler + Send + Sync>>::new();
        handlers.insert(
            ServerOperation::ConnectServerRequest as i8,
            Box::new(ConnectHandler),
        );
        handlers.insert(ServerOperation::MoveRequest as i8, Box::new(MoveHandler));
        let se = Self {
            game_state,
            handlers,
        };
        return se;
    }

    pub async fn handle(&self, packet: Packet, tcp_stream: Arc<RwLock<TcpStream>>) {
        let player: Player;
        let game_state = self.game_state.write().await;
        if packet.operation as i8 != 0 {
            {
                let mut identity = Identity::authenticate(packet.token);
                if identity.clone().is_none() {
                    return println!("Unauthorized");
                }
                let players_lock = game_state.players.read().await;
                let found_player = players_lock
                    .iter()
                    .find(|p| p.uid == identity.as_mut().unwrap().uid)
                    .cloned();
                match found_player {
                    Some(p) => {
                        player = p;
                    }
                    None => {
                        panic!(
                            "player isn't in game state {}",
                            identity.as_mut().unwrap().uid,
                        )
                    }
                }
            }
        } else {
            player = Player::new(tcp_stream);
            game_state.add_player(player.clone()).await;
        }
        if let Some(handler) = self.handlers.get(&(packet.operation as i8)) {
            println!("handle {}", packet.operation as i8);
            handler
                .handle(game_state.clone(), Some(&packet.content), player)
                .await;
        } else {
            println!("No handler for operation {}", packet.operation as i8);
        }
    }

    pub async fn remove_connection(&self, stream: &Arc<RwLock<TcpStream>>) {
        let mut player_to_remove = None;
        {
            let game_state = self.game_state.write().await;
            let players = game_state.players.read().await;
            for player in players.iter() {
                // Await `peer_addr()` for each player
                if player.peer_addr().await == stream.clone().read().await.peer_addr().unwrap() {
                    player_to_remove = Some(player.clone());
                    break; // Exit early once we find the matching player
                }
            }
        }
        if let Some(player) = player_to_remove {
            let game_state = self.game_state.write().await;
            game_state.remove_player(player).await;
        }
    }
}
