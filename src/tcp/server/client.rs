
use random_string::generate;
use std::{net::TcpStream, sync::Arc};


pub struct Client{
    tcp_stream: Arc<TcpStream>,
    uid : String
}

impl  Client {
    pub fn new(tcp_stream : TcpStream) -> Self{
        let charset = "abcdefghijklmnopqrstuvwxyz";
        Self{
            tcp_stream: Arc::new(tcp_stream),
            uid: generate(32, charset)
        }   
    }
    
    
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            tcp_stream: Arc::clone(&self.tcp_stream), // Clone sécurisé du TcpStream
            uid: self.uid.clone(),
        }
    }
}