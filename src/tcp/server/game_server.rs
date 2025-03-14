use std::fmt::format;

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    max_allowed_connections: i16,
    port: i16,
    is_running: bool,
}

impl Server {
    pub fn new(max_allowed_connections: Option<i16>, port: Option<i16>) -> Self {
        Self {
            max_allowed_connections: max_allowed_connections.unwrap_or(5000),
            port: port.unwrap_or(3333),
            is_running: true,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("{}:{}", "127.0.0.1", self.port);
        print!("Starting server on {}", address);
        let listener = TcpListener::bind(address).await.unwrap();

        loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();
            self.handle_client(socket).await;
        }
    }

    async fn handle_client(&self, socket: TcpStream) {
        // The `Connection` lets us read/write redis **frames** instead of
        // byte streams. The `Connection` type is defined by mini-redis.
        let mut connection = Connection::new(socket);

        if let Some(frame) = connection.read_frame().await.unwrap() {
            println!("GOT: {:?}", frame);

            // Respond with an error
            let response = Frame::Error("unimplemented".to_string());
            connection.write_frame(&response).await.unwrap();
        }
    }
}
