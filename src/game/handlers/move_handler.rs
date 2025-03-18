use std::sync::{Arc, Mutex};

use crate::game::{
    game_state::{GameState, Notify},
    player::Player,
};
use serde::{Deserialize, Serialize};

use super::handler::Handler;

pub struct MoveHandler;

impl Handler for MoveHandler {
    fn handle(&self, _game_state: &Arc<Mutex<GameState>>, data: &str, player: Player) {
        let move_request_obj: MoveHandlerRequest = serde_json::from_str(&data).unwrap();
        println!("asked direction {}", move_request_obj.direction as i8);
        let mut player_clone = player.clone();
        match move_request_obj.direction {
            MoveDirection::UP => player_clone.x += 10.0,
            MoveDirection::DOWN => player_clone.x -= 10.0,
            MoveDirection::LEFT => player_clone.y -= 10.0,
            MoveDirection::RIGHT => player_clone.y += 10.0,
        }
        let message = serde_json::to_string(&move_request_obj).unwrap();
        let game_state = _game_state.lock().unwrap();
        game_state.notify(player.stream, message, Notify::All);
    }
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
