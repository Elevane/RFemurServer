use std::sync::{Arc, Mutex};

use crate::{
    game::{game_state::GameState, player::Player},
    tcp::{server::packet::Packet, server_operation::ServerOperation},
};
use serde::{Deserialize, Serialize};

use super::handler::Handler;

pub struct MoveHandler;

impl Handler for MoveHandler {
    fn handle(&self, _game_state: GameState, _data: Option<&str>, player: Player) {
        let data = _data.expect("Could not move operation as data was null");
        let move_request_obj: MoveHandlerRequest =
            serde_json::from_str(data).expect("Failed to parse MoveHandlerRequest JSON");
        println!("asked direction {}", move_request_obj.direction as i8);

        match move_request_obj.direction {
            MoveDirection::UP => player.clone().x += 10.0,
            MoveDirection::DOWN => player.clone().x -= 10.0,
            MoveDirection::LEFT => player.clone().y -= 10.0,
            MoveDirection::RIGHT => player.clone().y += 10.0,
        }
        let data = serde_json::to_string(&move_request_obj).unwrap();

        notify_player(&player, &data);

        notify_other_players(player, _game_state.players, data);
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MoveHandlerRequest {
    direction: MoveDirection,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MoveDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
