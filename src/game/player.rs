use rand::Rng;
use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Player {
    pub stream: Arc<Mutex<TcpStream>>,
    pub x: f32,
    pub y: f32,
    pub uid: String,
}

impl Player {
    pub(crate) fn new(tcp_stream: TcpStream) -> Self {
        let mut rng = rand::rng();
        Self {
            stream: Arc::new(Mutex::new(tcp_stream)),
            x: rng.random_range(0.0..=500.0),
            y: rng.random_range(0.0..=500.0),
            uid: generate_id(),
        }
    }

    pub fn send_response(&self, packet: String) {
        {
            let _ = self.stream.lock().unwrap().write(packet.as_bytes());
        }
    }

    pub fn get_peer_addr(&self) -> SocketAddr {
        {
            return self.stream.lock().unwrap().peer_addr().unwrap();
        }
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
