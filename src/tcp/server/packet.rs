use std::str::Split;

use crate::tcp::server_operation::ServerOperation;

pub struct Packet {
    pub token: String,
    pub operation: ServerOperation,
    pub content: String,
}

impl Packet {
    pub fn decode(content: Split<'_, &str>) -> Option<Packet> {
        let mut count = 0;
        let mut token: &str = "";
        let mut operation: &str = "";
        let mut packet_content: &str = "";
        let parts: Vec<&str> = content.clone().collect();
        for part in content {
            match count {
                0 => token = part,
                1 => operation = part,
                2 => packet_content = part,
                _ => println!("invalid packet format size : {}", parts.len()),
            }
            count += 1;
        }

        let int = operation.trim().parse().unwrap();
        let opt = ServerOperation::decode(int);
        let packet = match opt {
            Some(operation) => {
                Packet::new(token.to_string(), operation, packet_content.to_string())
            }
            None => Packet::new(token.to_string(), opt?, packet_content.to_string()),
        };
        Some(packet)
    }

    fn new(token: String, opt: ServerOperation, packet_content: String) -> Self {
        Self {
            token: token.to_string(),
            operation: opt,
            content: packet_content.to_string(),
        }
    }

    pub fn encode(operation: ServerOperation, data: String, token: Option<String>) -> String {
        let opr = operation as i8;
        let tokn = token.unwrap_or("".to_string());
        return format!("{tokn}|{opr}|{data}").to_string();
    }
}
