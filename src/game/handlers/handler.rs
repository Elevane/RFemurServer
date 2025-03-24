use std::{future::Future, sync::Arc};

use tokio::sync::RwLock;

use crate::game::{game_state::GameState, player::Player};

// Trait pour définir un Handler
pub trait Handler {
    fn handle<'a>(
        &self,
        game_state: GameState,
        data: Option<&str>,
        player: Player,
    ) -> Box<dyn Future<Output = ()> + Send + 'a>;
}

pub trait AsyncHandler {
    fn handle_async<'a>(
        &self,
        game_state: GameState,
        data: Option<&str>,
        player: Player,
    ) -> Box<dyn Future<Output = ()> + Send + 'a>;
}
