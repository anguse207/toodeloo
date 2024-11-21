use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub nick: String,
    pub deleted_time: u64,
}

impl User {
    pub fn new(nick: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            nick: nick.into(),
            deleted_time: 0,
        }
    }
}
