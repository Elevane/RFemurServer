use std::sync::{Arc, Mutex};

use super::player::Player;

#[derive(Clone)]
pub struct GameState {
    pub players: Arc<Mutex<Vec<Player>>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn remove_player(&self, player: Player) {
        println!("removing player from game_state");
        let mut players_lock = self.players.lock().unwrap();
        println!("locked player from game_state");
        players_lock.retain(|p| p.uid != player.uid);
    }

    pub fn add_player(&self, player: Player) {
        let mut players_lock = self.players.lock().unwrap();
        players_lock.push(player);
    }
}
