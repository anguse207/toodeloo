mod user_impl;
// mod list_impl;
// mod task_impl;

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
        println!("Creating tables");
        sqlx::query(
            r#"
            CREATE TABLE Users (
                id TEXT PRIMARY KEY,
                nick TEXT NOT NULL,
                token TEXT,
                deleted_time INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        println!("  Created Users Table");

        sqlx::query(
            r#"
            CREATE TABLE Lists (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                label TEXT NOT NULL,
                deleted_time INTEGER NOT NULL,
                FOREIGN KEY (user_id) REFERENCES Users(id) 
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        println!("  Created Lists Table");

        sqlx::query(
            r#"
            CREATE TABLE Tasks (
                id TEXT PRIMARY KEY,
                list_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                origin_time INTEGER NOT NULL,
                content TEXT NOT NULL,
                done BOOLEAN NOT NULL,
                snoozed_until INTEGER NOT NULL,
                deleted_time INTEGER NOT NULL,
                FOREIGN KEY (user_id) REFERENCES Users(id),
                FOREIGN KEY (list_id) REFERENCES Lists(id)

            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        println!("  Created Tasks Table\n");

        Ok(())
    }
}
