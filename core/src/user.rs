use sqlx::FromRow;
use uuid::Uuid;

use crate::task::Task;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub nick: String,
}

impl User {
    pub fn new(nick: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            nick: nick.into(),
        }
    }
}
