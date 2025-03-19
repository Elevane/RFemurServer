use std::{
    fmt,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex, MutexGuard},
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
        tcp_stream: MutexGuard<'_, TcpStream>,
        message: String,
        notify_scope: Notify,
    ) -> () {
        println!("Starting notifier {}", notify_scope);

        let sender_addr = match tcp_stream.peer_addr() {
            Ok(addr) => addr,
            Err(_) => {
                println!("Failed to get sender address");
                return;
            }
        };

        let to_notify: Vec<Arc<Mutex<TcpStream>>>;
        {
            println!("Trying to lock players...");
            let players = self.players.lock().expect("Failed to lock players list");
            println!("Players locked");

            if players.is_empty() {
                println!("No players to notify");
                return;
            }

            println!("Filtering players...");

            to_notify = players
                .iter()
                .filter_map(|player| {
                    let player_stream = player.stream.clone();
                    println!("Checking player...");
                
                    let player_addr = {
                        println!("Trying to lock player stream...");
                        let stream = match player_stream.lock() {
                            Ok(s) => {
                                println!("Player stream locked successfully.");
                                s
                            }
                            Err(e) => {
                                println!("Failed to lock player stream: {}", e);
                                return None;
                            }
                        };
                        println!("Trying to get peer address...");
                        match stream.peer_addr() {
                            Ok(addr) => {
                                println!("Got peer address: {}", addr);
                                Some(addr)
                            }
                            Err(e) => {
                                println!("Failed to get peer address: {}", e);
                                None
                            }
                        }
                    };
                
                    if let Some(addr) = player_addr {
                        if addr != sender_addr {
                            println!("Adding player to notify list...");
                            return Some(player_stream);
                        }
                    }
                
                    println!("Skipping player...");
                    None
                })
                
                .collect();

            println!("to notify initialized");
        }

        for stream in to_notify {
            if let Ok(mut player_stream) = stream.lock() {
                if let Err(e) = player_stream.write_all(message.as_bytes()) {
                    println!("Failed to send message: {}", e);
                } else {
                    println!("Message sent successfully.");
                }
            }
        }

        println!("Finished notify");
    }
}

pub enum Notify {
    All,
}

impl fmt::Display for Notify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Notify::All => write!(f, "Notify::All"),
        }
    }
}
