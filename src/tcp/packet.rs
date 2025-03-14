
use crate::tcp::server_operation::ServerOperation;

pub struct Packet{
    pub operation: ServerOperation,
    pub content: String
}

impl Packet{
    pub fn new(operation: ServerOperation, content: String) -> Self{
        Packet{operation, content}
    }
}
