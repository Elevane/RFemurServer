use rand::Rng;
use std::{future::IntoFuture, net::SocketAddr, sync::Arc};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::RwLock};

#[derive(Clone)]
pub struct Player {
    pub stream: Arc<RwLock<TcpStream>>,
    pub x: f32,
    pub y: f32,
    pub uid: String,
}

impl Player {
    pub(crate) fn new(tcp_stream: Arc<RwLock<TcpStream>>) -> Self {
        let mut rng = rand::rng();
        Self {
            stream: tcp_stream,
            x: rng.random_range(0.0..=500.0),
            y: rng.random_range(0.0..=500.0),
            uid: generate_id(),
        }
    }

    pub async fn peer_addr(&self) -> SocketAddr {
        return self.stream.read().await.peer_addr().unwrap();
    }
    pub async fn send_response(&self, packet: String) {
        // Lock the TcpStream for writing
        let mut stream = self.stream.write().await; // use `.await` to wait for the lock asynchronously

        // Write the response asynchronously
        if let Err(e) = stream.write_all(packet.as_bytes()).await {
            eprintln!("Failed to send response: {}", e);
        }
        println!("Response sent to player: {}", self.uid);
    }
}

pub fn generate_id() -> String {
    let charset = "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN123456789";
    let mut rng = rand::rng();
    let length = 16;
    let mut id = "".to_string();
    for _ in 1..length {
        let char = charset.as_bytes()[rng.random_range(0..charset.chars().count())] as char;
        id.push(char);
    }
    return id;
}
