mod list_impl;
mod task_impl;
mod user_impl;

use anyhow::Result;
use sqlx::SqlitePool;
use std::fs::{self, File};

#[derive(Debug, Clone)]
pub struct Tank {
    pool: SqlitePool,
}

impl Tank {
    pub async fn new(db: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            pool: SqlitePool::connect(db.as_ref()).await?,
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
                origin_time INTEGER NOT NULL,
                title TEXT,
                content TEXT,
                done BOOLEAN NOT NULL,
                snoozed_until INTEGER NOT NULL,
                deleted_time INTEGER NOT NULL,
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

// For debugging / testing purposes
pub async fn init_db(db_file: impl AsRef<str>) -> Result<Tank> {
    if fs::metadata(db_file.as_ref()).is_ok() {
        fs::remove_file(db_file.as_ref())?;
    }

    File::create(db_file.as_ref())?;

    // Connect and create tables
    let tank = Tank::new(&format!("sqlite://{}", db_file.as_ref())).await?;
    tank.create_tables().await?;

    Ok(tank)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn general_tests() {
    //     let tank = init_db("testing.db").await.unwrap();

    //     // Create 2 user
    //     let john_id = tank.new_user("John Tez").await.unwrap();
    //     println!("Created user with ID: {}", john_id);

    //     let frank_id = tank.new_user("Frank Lamps").await.unwrap();
    //     println!("Created user with ID: {}", frank_id);

    //     // Get all users
    //     println!("Users: {:?}", tank.get_users().await.unwrap());

    //     // Get users
    //     let _john = tank.get_user(&john_id.clone()).await.unwrap();
    //     let _frank = tank.get_user(&frank_id.clone()).await.unwrap();

    //     // Create lists
    //     let chores_id = tank.new_list(&john_id, "Chores").await.unwrap();
    //     let features_id = tank.new_list(&frank_id, "Software Features").await.unwrap();

    //     println!("John lists: {:?}", tank.get_lists(&john_id).await.unwrap());
    //     println!(
    //         "Frank lists: {:?}",
    //         tank.get_lists(&frank_id).await.unwrap()
    //     );

    //     // Add Chores
    //     let shopping_task = tank
    //         .new_task(&chores_id, "Shopping List", "Milk, Eggs, Cheese")
    //         .await
    //         .unwrap();
    //     let laundry_task = tank
    //         .new_task(&chores_id, "Laundry", "Wash clothes and hang them to dry")
    //         .await
    //         .unwrap();
    //     // let lorem_task = tank
    //     //     .new_task(&chores_id, "Lorem", TEST_STRING)
    //     //     .await.unwrap();

    //     // Add Features
    //     let login_task = tank
    //         .new_task(&features_id, "Login", "Implement login feature")
    //         .await
    //         .unwrap();
    //     let logout_task = tank
    //         .new_task(&features_id, "Logout", "Implement logout feature")
    //         .await
    //         .unwrap();
    //     let register_task = tank
    //         .new_task(&features_id, "Register", "Implement register feature")
    //         .await
    //         .unwrap();

    //     println!("Chores: {:?}", tank.get_tasks(&chores_id).await.unwrap());
    //     println!(
    //         "Features: {:?}",
    //         tank.get_tasks(&features_id).await.unwrap()
    //     );

    //     // Update user / Mark deleted
    //     // nick.deleted_time = get_timestamp();
    //     // tank.update_user(&nick_id.clone(), &nick).await.unwrap();

    //     // Remove chores
    //     tank.remove_task(&shopping_task).await.unwrap();
    //     tank.remove_task(&laundry_task).await.unwrap();
    //     // tank.remove_task(&lorem_task).await.unwrap();

    //     // Remove features
    //     tank.remove_task(&login_task).await.unwrap();
    //     tank.remove_task(&logout_task).await.unwrap();
    //     tank.remove_task(&register_task).await.unwrap();

    //     // Remove lists
    //     tank.remove_list(&chores_id).await.unwrap();
    //     tank.remove_list(&features_id).await.unwrap();

    //     // Remove users
    //     tank.remove_user(&john_id).await.unwrap();
    //     tank.remove_user(&frank_id).await.unwrap();

    //     // Get all users
    //     println!("Users: {:?}", tank.get_users().await.unwrap());
    // }
}
