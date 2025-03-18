use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use super::player::Player;

#[derive(Clone)]
pub struct GameState {
    pub players: Arc<Mutex<Vec<Player>>>,
}

impl GameState {
    pub(crate) fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(Vec::<Player>::new())),
        }
    }

    pub(crate) fn notify(
        &self,
        tcp_stream: TcpStream,
        message: String,
        notify_scope: Notify,
    ) -> () {
        println!(
            "Notifying {} : {:?}",
            notify_scope as i8,
            tcp_stream.try_clone().unwrap().peer_addr()
        );
        for cl in self.players.lock().unwrap().iter_mut() {
            if cl.stream.peer_addr().unwrap() != tcp_stream.peer_addr().unwrap() {
                cl.stream.write(message.as_bytes()).unwrap();
            }
        }
    }
}

pub enum Notify {
    All,
}
