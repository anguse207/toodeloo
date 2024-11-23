use sqlx::FromRow;
use uuid::Uuid;

use crate::tank_traits::ID;

#[derive(FromRow, Debug, PartialEq)]
pub struct List {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub deleted_time: u64,
}

impl List {
    pub fn new(user_id: ID, label: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            label: label.into(),
            deleted_time: 0,
        }
    }
}
