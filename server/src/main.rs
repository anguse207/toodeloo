mod tank_sqlite;

use std::fs::{self, File};

use anyhow::{Ok, Result};
use tank_sqlite::Tank;
use toodeloo_core::{tank_traits::UserTank, timing::get_timestamp};

#[tokio::main]
async fn main() -> Result<()> {
    let tank = init_db("testing.db").await?;

    // Create 2 user
    let nick_id = tank.new_user("Nick").await?;
    println!("Created user with ID: {}", nick_id);

    // Get all users
    println!("Users: {:?}", tank.get_users().await?);
    
    // Get user
    let mut nick = tank.get_user(&nick_id.clone()).await?;

    // Update user / Mark deleted
    nick.deleted_time = get_timestamp();
    tank.update_user(&nick_id.clone(), &nick).await?;

    println!("Users: {:?}", tank.get_users().await?);

    // Remove user
    tank.remove_user(&nick_id).await?;

    // Get all users
    println!("Users: {:?}", tank.get_users().await?);

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
