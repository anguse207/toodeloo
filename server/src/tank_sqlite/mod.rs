mod list_impl;
mod task_impl;
mod user_impl;

use anyhow::Result;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Tank {
    pool: SqlitePool,
}

#[allow(unused)]
impl Tank {
    pub async fn new(db: &str) -> Result<Self> {
        Ok(Self {
            pool: SqlitePool::connect(db).await?,
        })
    }

    pub async fn new_from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // Verify with `$ sqlite3 <db_name>.db` `sqlite> .tables`
    pub async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE users (
                id TEXT PRIMARY KEY,
                nick TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE tasks (
                id TEXT PRIMARY KEY, -- UUIDs stored as text
                list_id TEXT NOT NULL, -- Foreign key to a list
                user_id TEXT NOT NULL, -- Foreign key to a user
                origin_time INTEGER NOT NULL, -- Unix timestamp (u64)
                content TEXT NOT NULL, -- Assuming Content is stored as text
                done BOOLEAN NOT NULL, -- Boolean indicating if the task is done
                snoozed_until INTEGER -- Nullable, for optional snooze time
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
