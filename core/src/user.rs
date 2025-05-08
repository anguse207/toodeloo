use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, PartialEq, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub pass: String,
    pub deleted_time: i64,
}

// #[derive(Deserialize, Debug)]
// pub struct UpdateUser {
//     pub name: String,
//     pub pass: String,
//     pub deleted_time: u64,
// }

impl User {
    pub fn new(name: impl Into<String>, pass: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            pass: pass.into(),
            deleted_time: 0,
        }
    }
}

// impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for User {
//     fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, Error> {
//         Ok(User {
//             id: Uuid::parse_str(row.try_get::<String, _>("id")?.as_str())
//                 .map_err(|_| Error::Decode("invalid UUID format".into()))?,
//             name: row.try_get("name")?,
//             pass: row.try_get("pass")?,
//             deleted_time: row.try_get("deleted_time")?,
//         })
//     }
// }
