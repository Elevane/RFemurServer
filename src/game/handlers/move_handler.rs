use std::{future::Future, pin::Pin, sync::Arc};

use crate::{
    game::{game_state::GameState, player::Player},
    tcp::{server::packet::Packet, server_operation::ServerOperation},
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::handler::{AsyncHandler, Handler};

pub struct MoveHandler;

impl Handler for MoveHandler {
    fn handle<'a>(
        &self,
        game_state: GameState,
        _data: Option<&'a str>,
        player: Player,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let data = _data.expect("Missing move data"); // à améliorer
            let move_request_obj: MoveHandlerRequest =
                serde_json::from_str(data).expect("Invalid MoveHandlerRequest");

            let mut player = player;
            match move_request_obj.direction {
                MoveDirection::UP => player.x += 10.0,
                MoveDirection::DOWN => player.x -= 10.0,
                MoveDirection::LEFT => player.y -= 10.0,
                MoveDirection::RIGHT => player.y += 10.0,
            }

            let response = serde_json::to_string(&move_request_obj).unwrap();
            notify_player(&player, &response).await;
            notify_other_players(player, game_state.players, response).await;
        })
    }
}
async fn notify_other_players(
    player: Player,
    players_lock: Arc<RwLock<Vec<Player>>>,
    data: String,
) {
    let other_packet = Packet::encode(
        ServerOperation::ConnectServerOtherPlayer,
        data.clone(),
        None,
    );
    println!("-Generated response");

    for p in players_lock.read().await.iter() {
        if p.uid != player.uid {
            p.send_response(other_packet.clone()).await;
        }
    }
}

async fn notify_player(player: &Player, data: &String) {
    let packet = Packet::encode(
        ServerOperation::ConnectServerRequestTokenResponse,
        data.clone(),
        Some("token".to_string()),
    );
    let player_clone = player.clone();
    player_clone.send_response(packet).await;
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
