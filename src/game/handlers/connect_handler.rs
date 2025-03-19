use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{
    game::{
        game_state::{GameState, Notify},
        player::Player,
    },
    tcp::{server::packet::Packet, server_operation::ServerOperation},
};

use super::handler::Handler;
pub struct ConnectHandler;

impl Handler for ConnectHandler {
    fn handle(&self, _game_state: &Arc<Mutex<GameState>>, data: &str, player: Player) {
        {
            let stream_lock = player.stream.lock().unwrap();
            println!(
                "Handling connection request from {} : {}",
                stream_lock.peer_addr().unwrap(),
                data.trim()
            );

            let game_state = _game_state.lock().unwrap();
            // Verrouille ensuite la liste des joueurs
            {
                let mut players_lock = game_state.players.lock().unwrap();
                players_lock.push(player.clone());
            }
            let data = format!(
                "{{\"x\": {},\"y\": {},\"uid\": \"{}\"}}",
                player.x, player.y, player.uid
            );

            let packet = Packet::encode(
                ServerOperation::ConnectServerRequestTokenResponse,
                data.clone(),
                Some("token".to_string()),
            );
            let _ = stream_lock
                .try_clone()
                .expect("Erreur lors du clonage du stream")
                .write_all(packet.to_string().as_bytes());

            let other_packet = Packet::encode(
                ServerOperation::ConnectServerOtherPlayer,
                data.clone(),
                None,
            );
            println!("-Generated response");
            game_state.notify(stream_lock, other_packet, Notify::All);
            println!("Handled connection request");
        }
    }
}
