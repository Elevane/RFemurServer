use std::fmt::{self};

#[derive(Debug, Copy, Clone)]
pub enum ServerOperation {
    ConnectServerRequest,
    ConnectServerRequestTokenResponse,
    ConnectGameRequest,
}

impl ServerOperation {
    pub(crate) fn decode(operation: i32) -> Option<Self> {
        return match operation {
            0 => Some(ServerOperation::ConnectServerRequest),
            1 => Some(ServerOperation::ConnectServerRequestTokenResponse),
            2 => Some(ServerOperation::ConnectGameRequest),
            _ => None,
        };
    }
}

impl fmt::Display for ServerOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerOperation::ConnectServerRequest => write!(f, "ConnectServerRequest"),
            ServerOperation::ConnectServerRequestTokenResponse => {
                write!(f, "ConnectServerRequestTokenResponse")
            }
            ServerOperation::ConnectGameRequest => write!(f, "ConnectGameRequest"),
        }
    }
}
