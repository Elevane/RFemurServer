use std::fmt::{self};

#[derive(Debug, Copy, Clone)]
pub enum ServerOperation {
    ConnectServerRequest,
    ConnectServerRequestTokenResponse,
    ConnectGameRequest,
    MoveRequest,
    MoveResponse,
}

impl ServerOperation {
    pub(crate) fn decode(operation: i32) -> Option<Self> {
        return match operation {
            0 => Some(ServerOperation::ConnectServerRequest),
            1 => Some(ServerOperation::ConnectServerRequestTokenResponse),
            2 => Some(ServerOperation::ConnectGameRequest),
            3 => Some(ServerOperation::MoveRequest),
            4 => Some(ServerOperation::MoveResponse),
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
            ServerOperation::MoveRequest => write!(f, "MoveRequest"),
            ServerOperation::MoveResponse => write!(f, "MoveResponse"),
        }
    }
}
