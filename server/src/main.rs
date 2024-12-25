mod tank_sqlite;

use std::fs::{self, File};

use anyhow::{Ok, Result};
use tank_sqlite::Tank;
use toodeloo_core::tank_traits::{ListTank, TaskTank, UserTank};

#[tokio::main]
async fn main() -> Result<()> {
    let tank = init_db("testing.db").await?;

    // Create 2 user
    let nick_id = tank.new_user("John Doe").await?;
    println!("Created user with ID: {}", nick_id);

    // Get all users
    println!("Users: {:?}", tank.get_users().await?);

    // Get user
    let _nick = tank.get_user(&nick_id.clone()).await?;

    // Create list
    let list_id = tank.new_list(&nick_id, "Groceries").await?;
    println!("Lists: {:?}", tank.get_lists(&nick_id).await?);

    // Create task
    let task_id = tank
        .new_task(&list_id, "Shopping List", "Milk, Eggs, Cheese")
        .await?;
    println!("Tasks: {:?}", tank.get_tasks(&list_id).await?);

    // Update user / Mark deleted
    // nick.deleted_time = get_timestamp();
    // tank.update_user(&nick_id.clone(), &nick).await?;

    // Remove Task
    tank.remove_task(&task_id).await?;
    // Remove list
    tank.remove_list(&list_id).await?;
    // Remove user
    tank.remove_user(&nick_id).await?;

    // Get all users
    println!("Users: {:?}", tank.get_users().await?);

    Ok(())
}

// For debugging / testing purposes
async fn init_db(db_file: &str) -> Result<Tank> {
    if fs::metadata(db_file).is_ok() {
        fs::remove_file(db_file)?;
    }

    File::create(db_file)?;

    // Connect and create tables
    let tank = Tank::new(&format!("sqlite://{}", db_file)).await?;
    tank.create_tables().await?;

    Ok(tank)
}
