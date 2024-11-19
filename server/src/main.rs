mod tank;

use anyhow::{Ok, Result};
use sqlx::SqlitePool;
use tank::Tank;
use toodeloo_core::tank_traits::TaskTank;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite://testing.db").await?;
    let tank = Tank::new(pool);
    let nick_id = tank.new_user("Nick".to_string()).await?;
    println!("Created user with ID: {}", nick_id);

    Ok(())
}
