use std::net::TcpStream;

pub struct Player {
    pub stream: TcpStream,
    pub x: f32,
    pub y: f32,
    pub uid: String,
}

impl Player {
    pub(crate) fn new(tcp_stream: TcpStream) -> Self {
        let charset = "azertyuiopqsdfghjklmwxcvbn,;:!ù*$^";
        Self {
            stream: tcp_stream,
            x: 0.00,
            y: 0.00,
            uid: random_string::generate(25, charset),
        }
    }
}
// Remove the derive and implement Clone manually
impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            uid: self.uid.clone(),
            stream: self.stream.try_clone().expect("Failed to clone TcpStream"),
            x: self.x,
            y: self.y,
        }
    }
}
