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
            // On verrouille la liste des joueurs
            let players = match self.players.lock() {
                Ok(players) => players,
                Err(_) => {
                    println!("Failed to lock players list");
                    return;
                }
            };

            if players.is_empty() {
                println!("No players to notify");
                return;
            }

            println!("Players lock initialized");

            // Filtrage des joueurs à notifier
            to_notify = players
                .iter()
                .filter_map(|player| {
                    let player_stream = player.stream.clone();

                    // Tenter de verrouiller le stream du joueur
                    println!("Trying to lock player stream...");
                    match player_stream.lock() {
                        Ok(stream) => {
                            // Nous avons verrouillé le stream avec succès
                            match stream.peer_addr() {
                                Ok(addr) => {
                                    if addr != sender_addr {
                                        println!("Adding player to notify list...");
                                        return Some(player_stream.clone());
                                    }
                                }
                                Err(_) => {
                                    println!("Failed to get player address");
                                }
                            }
                        }
                        Err(_) => {
                            println!("Failed to lock player stream.");
                        }
                    }

                    None
                })
                .collect();

            println!("to_notify initialized");
        }

        // Étape 2 : Envoyer les messages (verrouiller les `TcpStream` un par un)
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
