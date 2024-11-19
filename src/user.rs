use uuid::Uuid;

use crate::task::Task;

pub struct User {
    pub id: Uuid,
    pub nick: String,
}

impl User {
    pub fn new(nick: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            nick,
        }
    }
}
