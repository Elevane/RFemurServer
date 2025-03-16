use std::fmt;


#[derive(Debug)]
pub enum ServerOperation{
    ConnectServer,
    ConnectGame,
}
impl ServerOperation {
    pub(crate) fn decode(operation: i32) -> Option<Self> {
        return match operation {
            0 => Some(ServerOperation::ConnectServer),
            1 => Some(ServerOperation::ConnectGame),
            _ => None, 
        };
    }
}

impl fmt::Display for ServerOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerOperation::ConnectServer => write!(f, "ConnectServer"),
            ServerOperation::ConnectGame => write!(f, "ConnectGame"),
        }
    }
}
