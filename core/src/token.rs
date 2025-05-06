use sqlx::{Error, FromRow, Row};
use uuid::Uuid;

use crate::timing::{self, get_timestamp};

pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expiry: u64,
    pub revoked: bool,
}

impl Token {
    pub fn new(user_id: impl Into<Uuid>, duration_seconds: u64) -> Self {
        let expiry = get_timestamp() + duration_seconds;
        Self {
            id: Uuid::new_v4(),
            user_id: user_id.into(),
            expiry,
            revoked: false,
        }
    }
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for Token {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, Error> {
        Ok(
            Token {
                id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())
                    .map_err(|_| Error::Decode("invalid UUID format".into()))?,
                user_id: Uuid::parse_str(row.try_get::<String, _>("user_id")?.as_str())
                    .map_err(|_| Error::Decode("invalid UUID format".into()))?,
                expiry: row.try_get("expiry")?,
                revoked: row.try_get("revoked")?,
            },
        )
    }
}