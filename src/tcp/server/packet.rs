
use std::{borrow::Cow, error::Error, ptr::null, str::Split};

use crate::tcp::server_operation::ServerOperation;

pub struct Packet{
    pub token:String,
    pub operation: ServerOperation,
    pub content: String
}

impl Packet{
    pub fn decode(content: Split<'_, &str>) -> (){
        let mut count = 0;
        let mut token: &str = "";
        let mut operation: &str = "";
        let mut packet_content: &str = "";
        for part in content{
            if(count == 0){
                 token = part;
            }
            if(count == 1)
            {
                 operation = part;
            }
            if(count == 2)
            {
                 packet_content = part;
            }
            
            count +=1;
        }
        println!("decoded packet {}",  operation);
        let int = operation.trim().parse().unwrap();
        let opt = ServerOperation::decode(int);
        if !opt.is_none()
        {
            println!("Operation decoded : {}", opt.unwrap());
        }
    }
    
}
