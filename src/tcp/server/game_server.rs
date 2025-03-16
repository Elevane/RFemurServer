

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};

use super::{client::Client, packet::Packet};

pub struct Server {
    max_allowed_connections:i16,
    port: i16,
    connections: Arc<Mutex<Vec<Client>>>
}

impl Server {
    pub fn new( max_allowed_connections: Option<i16>, port: Option<i16>) -> Self {
        Self {
            max_allowed_connections : max_allowed_connections.unwrap_or(5000),
            port: port.unwrap_or(3333),
            connections: Arc::new(Mutex::new(Vec::new())), 
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("{}:{}", "127.0.0.1", self.port);
        println!("Starting server on {}", address);
        let listener = TcpListener::bind(address).unwrap();

        loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().unwrap();
            let _ = &self.handle_client(socket);
        }
    }

    fn handle_client(&mut self, mut  stream: TcpStream) {
        
        let peer_addr = stream.peer_addr().unwrap();
        println!("Client connecté: {}", peer_addr);
        let mut buffer = [0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Client déconnecté: {}", peer_addr);
                    break;
                }
                Ok(_) => {
                    let message = String::from_utf8_lossy(&buffer);
                    println!("Message reçu de {}: {}", peer_addr, message);
                    
                    self.decode_message(message);
                    {
                        let mut clients_guard = self.connections.lock().unwrap();
                        let client = Client::new(stream.try_clone().unwrap());
                        clients_guard.push(client); 
                    }
                    
                }
                Err(e) => {
                    println!("Erreur avec le client {}: {}", peer_addr, e);
                    
                }
            }
        }
      }
      pub fn decode_message(&mut self, message: std::borrow::Cow<'_, str>) -> () {
        let parts = message.split("|");
        if(parts.clone().count() != 3 as usize ){
            panic!("Incorrect tcp format");
        }
        let packet = Packet::decode(parts);
    }
}


