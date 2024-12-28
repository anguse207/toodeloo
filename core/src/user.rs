use serde::Serialize;
use sqlx::{Error, FromRow, Row};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize)]
pub struct User {
    pub id: Uuid,
    pub nick: String,
    pub token: Uuid,
    pub deleted_time: u64,
}

impl User {
    pub fn new(nick: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            nick: nick.into(),
            token: Uuid::new_v4(),
            deleted_time: 0,
        }
    }
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for User {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, Error> {
        Ok(User {
            id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())
                .map_err(|_| Error::Decode("invalid UUID format".into()))?,
            nick: row.try_get("nick")?,
            token: Uuid::parse_str(row.try_get::<String, _>("token")?.as_str())
                .map_err(|_| Error::Decode("invalid UUID format".into()))?,
            deleted_time: row.try_get("deleted_time")?,
        })
    }
}
