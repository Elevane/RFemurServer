use std::sync::{Arc, Mutex};

use crate::game::{game_state::GameState, player::Player};

// Trait pour définir un Handler
pub trait Handler {
    fn handle(&self, game_state: &Arc<Mutex<GameState>>, data: &str, player: Player);
}
