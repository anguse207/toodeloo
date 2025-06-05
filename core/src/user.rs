use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Type, Copy, Serialize)]
#[sqlx(type_name = "oauth_provider")] // Matches the PostgreSQL enum type name
#[sqlx(rename_all = "lowercase")] // Matches the PostgreSQL enum values
pub enum OauthProvider {
    Discord,
    Google,
    Github,
    Apple,
}

#[derive(Debug, PartialEq, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub oauth_id: String,
    pub oauth_provider: OauthProvider,
    pub nickname: String,
    pub deleted_time: i64,
}

impl User {
    pub fn new(
        oauth_id: impl Into<String>,
        oauth_provider: OauthProvider,
        nickname: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            oauth_id: oauth_id.into(),
            oauth_provider,
            nickname: nickname.into(),
            deleted_time: 0,
        }
    }
}
