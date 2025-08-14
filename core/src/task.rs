use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::timing::get_timestamp;

#[derive(Debug, PartialEq, FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub origin_time: i64,
    pub title: String,
    pub content: String,
    pub done: bool,
    pub snoozed_until: i64,
    pub deleted_time: i64,
    pub last_activity: i64,
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
            last_activity: 0,
        }
    }
}
