use crate::game::{game_state::GameState, player::Player};

// Trait pour définir un Handler
pub trait Handler {
    fn handle(&self, game_state: GameState, data: Option<&str>, player: Player);
}
