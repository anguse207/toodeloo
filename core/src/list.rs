use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, PartialEq, FromRow, Serialize)]
pub struct List {
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
    pub deleted_time: i64,
}

impl List {
    pub fn new(user_id: Uuid, label: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            label: label.into(),
            deleted_time: 0,
        }
    }
}
