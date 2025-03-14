

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    max_allowed_connections:i16,
    port: i16,
    connections: Vec<Connection>
}

impl Server {
    pub fn new(&self, max_allowed_connections: Option<i16>, port: Option<i16>) -> Self {
        Self {
            max_allowed_connections : max_allowed_connections.unwrap_or(50000),
            port: port.unwrap_or(3333),
            connections: Vec::new(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("{}:{}", "127.0.0.1", self.port);
        println!("Starting server on {}", address);
        let listener = TcpListener::bind(address).await.unwrap();

        loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();
            
            self.handle_client(socket).await;
        }
    }

    async fn handle_client(&self, socket: TcpStream) {
        let mut connection = Connection::new(socket);
        let size = {
            let locked_vec = self.connections.to_vec().lock().await;
            locked_vec.len()
        };
        if(size >  self.max_allowed_connections)
        {
            
            connection.write_frame("Too much connexion atm");
            socket.shutdown();
        }
        if let Some(frame) = connection.read_frame().await.unwrap() {
            println!("GOT: {:?}", frame);
            let response = Frame::Error("unimplemented".to_string());
            connection.write_frame(&response).await.unwrap();
        }
    }
}
