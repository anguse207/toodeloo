use sqlx::{Error, FromRow, Row};
use uuid::Uuid;

use crate::timing::get_timestamp;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub origin_time: u64,
    pub title: String,
    pub content: String,
    pub done: bool,
    pub snoozed_until: u64,
    pub deleted_time: u64,
}

impl Task {
    pub fn new(list_id: Uuid, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            list_id,
            origin_time: get_timestamp(),
            title,
            content,
            done: false,
            snoozed_until: 0,
            deleted_time: 0,
        }
    }
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for Task {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, Error> {
        Ok(Task {
            id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())
                .map_err(|_| Error::Decode("invalid UUID format".into()))?,
            list_id: Uuid::parse_str(row.try_get::<String, _>("list_id")?.as_str())
                .map_err(|_| Error::Decode("invalid UUID format".into()))?,
            origin_time: row.try_get("origin_time")?,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            done: row.try_get("done")?,
            snoozed_until: row.try_get("snoozed_until")?,
            deleted_time: row.try_get("deleted_time")?,
        })
    }
}
