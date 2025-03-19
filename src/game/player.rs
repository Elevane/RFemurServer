use rand::{self, Rng};
use std::{
    net::TcpStream,
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
        let charset = "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN123456789";
        let mut rng = rand::rng();
        Self {
            stream: Arc::new(Mutex::new(tcp_stream)),
            x: rng.random_range(0.0..=500.0),
            y: rng.random_range(0.0..=500.0),
            uid: random_string::generate(25, charset),
        }
    }
}
