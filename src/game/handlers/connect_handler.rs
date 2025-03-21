use crate::{
    game::{game_state::GameState, player::Player},
    tcp::{server::packet::Packet, server_operation::ServerOperation},
};
use std::sync::{Arc, Mutex};

use super::handler::Handler;

pub struct ConnectHandler;

impl Handler for ConnectHandler {
    fn handle(&self, game_state: GameState, _data: Option<&str>, player: Player) {
        println!("handling connecthandler");
        let data = format!(
            "{{\"x\": {},\"y\": {},\"uid\": \"{}\"}}",
            player.x, player.y, player.uid
        );

        notify_player(&player, &data);

        notify_other_players(player, game_state.players, data);

        println!("Handled connection request");
    }
}

fn notify_other_players(player: Player, players_lock: Arc<Mutex<Vec<Player>>>, data: String) {
    let other_packet = Packet::encode(
        ServerOperation::ConnectServerOtherPlayer,
        data.clone(),
        None,
    );
    println!("-Generated response");

    for p in players_lock.lock().unwrap().iter() {
        if p.uid != player.uid {
            p.send_response(other_packet.clone());
        }
    }
}

fn notify_player(player: &Player, data: &String) {
    let packet = Packet::encode(
        ServerOperation::ConnectServerRequestTokenResponse,
        data.clone(),
        Some("token".to_string()),
    );
    let player_clone = player.clone();
    player_clone.send_response(packet);
}
