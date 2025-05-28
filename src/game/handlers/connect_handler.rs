use tokio::sync::RwLock;

use crate::{
    game::{game_state::GameState, player::Player},
    tcp::{server::packet::Packet, server_operation::ServerOperation},
};
use std::{future::Future, pin::Pin, sync::Arc};

use super::handler::Handler;

pub struct ConnectHandler;

impl Handler for ConnectHandler {
    fn handle<'a>(
        &self,
        game_state: GameState,
        _data: Option<&'a str>,
        player: Player,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            println!("handling connecthandler");
            let data = format!(
                "{{\"x\": {},\"y\": {},\"uid\": \"{}\"}}",
                player.x, player.y, player.uid
            );

            notify_player(&player, &data).await;

            notify_other_players(player, game_state.players, data).await;

            println!("Handled connection request");
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
            println!("Send to player: {}", p.uid);
            p.send_response(other_packet.clone()).await;
        }
    }
}

async fn notify_player(player: &Player, data: &String) {
    let packet = Packet::encode(
        ServerOperation::ConnectServerRequestTokenResponse,
        data.clone(),
        Some(player.uid.to_string()),
    );
    let player_clone = player.clone();
    player_clone.send_response(packet).await;
}
