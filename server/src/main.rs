mod routes;
mod auth;

use anyhow::{Ok, Result};
use routes::create_router;

use tokio::net::TcpListener;
use toodeloo_tank::sqlite::Tank;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // setup tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Set max level to DEBUG
        .init();
    info!("Starting Logging...");

    // let tank = init_db("testing.db").await?;

    let tank = toodeloo_tank::sqlite::init_db("testing.db").await?;
    let app = create_router(tank).await;

    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("listening on {}", listener.local_addr()?);

    tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
    })
    .await?;

    Ok(())
}
