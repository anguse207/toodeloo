mod tank_sqlite;

use std::fs::{self, File};

use anyhow::{Ok, Result};
use tank_sqlite::Tank;
use toodeloo_core::tank_traits::UserTank;

#[tokio::main]
async fn main() -> Result<()> {
    let tank = init_db("testing.db").await?;

    // Create 2 user
    let nick_id = tank.new_user("Nick".to_string()).await?;
    println!("Created user with ID: {}", nick_id);

    let jerry_id = tank.new_user("Jerry".to_string()).await?;
    println!("Created user with ID: {}", jerry_id);

    // Get all users
    let users = tank.get_users().await?;
    println!("Users: {:?}", users);

    // Get a user
    let nick = tank.get_user(nick_id).await?;
    println!("Nick: {:?}", nick);

    Ok(())
}
// For debugging / testing purposes
async fn init_db(db: &str) -> Result<Tank> {
    if fs::metadata(db).is_ok() {
        fs::remove_file(db)?;
    }

    File::create(db)?;

    // Connect and create tables
    let tank = Tank::new(&format!("sqlite://{}", db)).await?;
    tank.create_tables().await?;

    Ok(tank)
}
