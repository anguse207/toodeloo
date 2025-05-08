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

// #[derive(Deserialize, Debug)]
// pub struct UpdateList {
//     pub user_id: Uuid,
//     pub label: String,
//     pub deleted_time: u64,
// }

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

// impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for List {
//     fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, Error> {
//         Ok(List {
//             id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())
//                 .map_err(|_| Error::Decode("invalid UUID format".into()))?,
//             user_id: Uuid::parse_str(row.try_get::<String, _>("user_id")?.as_str())
//                 .map_err(|_| Error::Decode("invalid UUID format".into()))?,
//             label: row.try_get("label")?,
//             deleted_time: row.try_get("deleted_time")?,
//         })
//     }
// }
