use sqlx::FromRow;
use uuid::Uuid;

use crate::timing::get_timestamp;

// TODO: Use a more complex type for content
pub type Content = String;

#[derive(FromRow, Debug)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub origin_time: u64,
    pub content: Content,
    pub done: bool,
    pub snoozed_until: u64,
    pub deleted_time: u64,
}

impl Task {
    pub fn new(list_id: Uuid, content: Content) -> Self {
        Self {
            id: Uuid::new_v4(),
            list_id,
            origin_time: get_timestamp(),
            content,
            done: false,
            snoozed_until: 0,
            deleted_time: 0,
        }
    }
}
