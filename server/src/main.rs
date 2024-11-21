mod tank_sqlite;

use std::fs::{self, File};

use anyhow::{Ok, Result};
use tank_sqlite::Tank;
use toodeloo_core::tank_traits::UserTank;

#[tokio::main]
async fn main() -> Result<()> {
    let tank = init_db("testing.db").await?;

    let nick_id = tank.new_user("Nick".to_string()).await?;
    println!("Created user with ID: {}", nick_id);

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
