mod traits_impl;

use anyhow::Result;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct Tank {
    pool: SqlitePool,
}

impl Tank {
    pub async fn new(db: &str) -> Result<Self> {
        Ok(Self {
            pool: SqlitePool::connect(db).await?,
        })
    }

    pub async fn new_from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }

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
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                origin_time INTEGER NOT NULL,
                content TEXT NOT NULL,
                done BOOLEAN NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
