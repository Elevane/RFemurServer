use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{
    game::{
        game_state::{GameState, Notify},
        player::Player,
    },
    tcp::{server::packet::{self, Packet}, server_operation::ServerOperation},
};

use super::handler::Handler;
pub struct ConnectHandler;

impl Handler for ConnectHandler {
    fn handle(&self, _game_state: &Arc<Mutex<GameState>>, data: &str, player: Player) {
        println!(
            "Handling connection request from {} {}",
            player.stream.peer_addr().unwrap(),
            data
        );
        
        let player_clone = player.clone();
        let game_state = _game_state.lock().unwrap();
        // Verrouille ensuite la liste des joueurs
        let mut players_lock = game_state.players.lock().unwrap();
        players_lock.push(player_clone);
        let player_clone_2 = player.clone();
        let data = format!(
            "{{\"x\": {},\"y\": {},\"uid\": \"{}\"}}",
            player_clone_2.x, player_clone_2.y, player_clone_2.uid
        );
        //let packet = Packet::encode(ServerOperation::MoveResponse, data);
        let packet = Packet::encode(ServerOperation::ConnectServerRequestTokenResponse, data, Some("token".to_string()));
        let _ = player
            .stream
            .try_clone()
            .expect("Erreur lors du clonage du stream")
            .write_all(packet.to_string().as_bytes());
       
        
        game_state.notify(player.stream, packet, Notify::All);
    }
}
