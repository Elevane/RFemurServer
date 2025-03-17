use std::time::SystemTime;

use chrono::{DateTime, Utc};

pub struct Identity {
    uid: String,
    //validity_date: DateTime<Utc>,
}
impl Identity {
    pub fn authenticate(token: String) -> Option<Identity> {
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        let i = Self { uid: token };
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("authenticating... {}{}", i.uid, formatted);
        return Some(i);
    }
}
