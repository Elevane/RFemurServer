use std::sync::Arc;

use tokio::sync::RwLock;

use super::player::Player;

#[derive(Clone)]
pub struct GameState {
    pub players: Arc<RwLock<Vec<Player>>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn remove_player(&self, player: Player) {
        println!("removing player from game_state");
        self.players.write().await.retain(|p| p.uid != player.uid);
        println!("locked player from game_state");
    }

    pub async fn add_player(&self, player: Player) {
        let mut players_lock = self.players.write().await;
        players_lock.push(player);
    }
}
