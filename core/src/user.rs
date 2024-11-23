use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub nick: String,
    pub deleted_time: u64,
}

impl User {
    pub fn new(nick: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            nick: nick.into(),
            deleted_time: 0,
        }
    }
}
