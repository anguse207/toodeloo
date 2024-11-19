use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

// TODO: Use a more complex type for content
type Content = String;

pub struct Task {
    id: Uuid,
    user_id: Uuid,
    time: u64,
    content: Content,
    done: bool,
}

impl Task {
    pub fn new(user_id: Uuid, content: Content) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            time: get_timestamp(),
            content,
            done: false,
        }
    }
}

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
