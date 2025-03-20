use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::game::{game_state::GameState, handlers::state_handler::StateHandler};

use super::packet::Packet;

pub struct Server {
    max_allowed_connections: i16,
    state_handler: StateHandler,
    port: i16,
    connections: i16,
    pub game_state: Arc<Mutex<GameState>>,
}

impl Server {
    pub fn new(max_allowed_connections: Option<i16>, port: Option<i16>) -> Self {
        let game_state = Arc::new(Mutex::new(GameState::new()));
        Self {
            game_state: Arc::clone(&game_state),
            state_handler: StateHandler::init(Arc::clone(&game_state)),
            max_allowed_connections: max_allowed_connections.unwrap_or(5000),
            port: port.unwrap_or(3333),
            connections: 0,
        }
    }

    pub fn start(&mut self) -> () {
        let address = format!("{}:{}", "127.0.0.1", self.port);
        println!("Starting server on {}", address);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(e) => {
                    println!("Erreur lors de l'acceptation de la connexion: {}", e);
                }
            }
        }
    }

    fn handle_client(&mut self, stream: TcpStream) {
        let mut buffer = [0; 512];
        let mut stream_clone = stream.try_clone().unwrap();
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(0) => {
                    println!(
                        "Client déconnecté: {}",
                        stream_clone.try_clone().unwrap().peer_addr().unwrap()
                    );
                    self.disconnect(stream_clone.try_clone().unwrap());
                    let _ = stream_clone.shutdown(std::net::Shutdown::Both);
                    break;
                }
                Ok(_) => {
                    println!("recieved message");
                    let message = String::from_utf8_lossy(&buffer);
                    self.decode_message(stream_clone.try_clone().unwrap(), message);
                    if self.connections <= self.max_allowed_connections {
                        self.connections += 1;
                    } else {
                        println!(
                            "reached maximum connections {}",
                            self.max_allowed_connections
                        )
                    }
                }
                Err(e) => {
                    println!(
                        "Erreur avec le client {}: {}",
                        stream_clone.try_clone().unwrap().peer_addr().unwrap(),
                        e
                    );
                    self.disconnect(stream_clone.try_clone().unwrap());
                    let _ = stream.shutdown(std::net::Shutdown::Both);
                }
            }
        }
    }

    pub fn disconnect(&self, stream: TcpStream) {
        // Verrouille le Mutex pour accéder à la liste des joueurs
        let game_state = self.game_state.lock().unwrap();
        let _ = stream.peer_addr().unwrap();
        // Verrouille ensuite la liste des joueurs
        let mut players = game_state.players.lock().unwrap();
        let mut index_to_remove = None;
        // Parcours des joueurs pour trouver celui à déconnecter
        for (index, player) in players.iter().enumerate() {
            {
                let player_stream = player.stream.lock().expect("Failed to lock player stream");
                if player_stream.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                    let _ = player_stream.shutdown(std::net::Shutdown::Both);
                    index_to_remove = Some(index);
                }
            }
        }

        // Retirer le joueur du vecteur si trouvé
        if let Some(index) = index_to_remove {
            players.remove(index);
        }
    }

    pub fn decode_message(&mut self, stream: TcpStream, message: std::borrow::Cow<'_, str>) -> () {
        let parts = message.split("|");
        if parts.clone().count() != 3 as usize {
            let _ = stream.shutdown(std::net::Shutdown::Both);
            return println!("Incorrect tcp format  : {}", message);
        }

        let packet = Packet::decode(parts).unwrap();
        self.state_handler.handle(packet, stream);
    }
}
