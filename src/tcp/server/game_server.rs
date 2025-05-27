use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use crate::game::{
    game_state::GameState,
    handlers::state_handler::{self, StateHandler},
};

use super::packet::Packet;

pub struct Server {
    pub max_allowed_connections: i16,
    pub port: i16,
    pub connections: i16,
    pub game_state: Arc<RwLock<GameState>>,
}

//TODO : probleme de multi connnections
impl Server {
    pub fn new(max_allowed_connections: Option<i16>, port: Option<i16>) -> Self {
        Self {
            game_state: Arc::new(RwLock::new(GameState::new())),
            max_allowed_connections: max_allowed_connections.unwrap_or(5000),
            port: port.unwrap_or(3333),
            connections: 0,
        }
    }

    pub async fn start(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:3333").await?;
        loop {
            let mut state_handler = StateHandler::init(Arc::clone(&self.game_state));
            let (mut stream, addr): (TcpStream, _) = listener.accept().await?;
            tokio::spawn(async move { Server::handle_client(stream, state_handler).await });
        }
    }

    pub async fn handle_client(stream: TcpStream, state_handler: StateHandler) {
        let player_stream = Arc::new(RwLock::new(stream)); // Créé une seule fois

        let mut buffer = [0; 1024];
        loop {
            let mut stream = player_stream.write().await; // Accès mutable pour lecture

            match stream.read(&mut buffer).await {
                Ok(0) => {
                    state_handler.remove_connection(&player_stream).await;
                    println!("Client déconnecté");
                    break;
                }
                Ok(_) => {
                    println!("Message reçu");
                    let message = String::from_utf8_lossy(&buffer);
                    println!("Reçu {}", message);

                    // Déverrouille le stream avant de passer à decode_message()
                    drop(stream); // Permet à d'autres tâches de lire le stream
                    println!("Déverrouillé le stream");
                    Server::decode_message(player_stream.clone(), message, &state_handler).await;
                }
                Err(e) => {
                    println!("Erreur avec le client: {}", e);
                    break;
                }
            }
        }
    }

    pub async fn decode_message(
        stream: Arc<RwLock<TcpStream>>,
        message: std::borrow::Cow<'_, str>,
        state_handler: &StateHandler,
    ) -> () {
        println!("Decode message ");
        let parts = message.split("|");
        if parts.clone().count() != 3 as usize {
            //drop(stream); //.shutdown(std::net::Shutdown::Both);
            return println!("Incorrect tcp format {}", message);
        }
        println!("to decode packet");
        let packet = Packet::decode(parts).unwrap();
        println!("decoded packet");
        state_handler.handle(packet, stream).await;
    }
}
