use std::time::{SystemTime, UNIX_EPOCH};

pub const WEEK: u64 = DAY * 7;
pub const DAY: u64 = HOUR * 24;
pub const HOUR: u64 = MINUTE * 60;
pub const MINUTE: u64 = 60;

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
