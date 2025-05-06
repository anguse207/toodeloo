mod auth;
mod routes;

use anyhow::{Ok, Result};
use routes::create_router;

use tokio::net::TcpListener;
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

    let listener = TcpListener::bind(":::1337").await?;
    println!("listening on {}", listener.local_addr()?);

    let app = create_router(tank).await;

    tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
        4000  })
    .await?;

    Ok(())
}
