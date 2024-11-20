mod tank;

use anyhow::{Ok, Result};
use tank::Tank;
use toodeloo_core::tank_traits::TaskTank;

#[tokio::main]
async fn main() -> Result<()> {
    let tank = Tank::new("sqlite://testing.db").await?;

    tank.create_tables().await?;

    let nick_id = tank.new_user("Nick".to_string()).await?;
    println!("Created user with ID: {}", nick_id);

    Ok(())
}
