mod routes;

use anyhow::{Ok, Result};
use routes::create_router;

use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // // setup tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Set max log level
        .with_target(true) // Include log targets (e.g., crate/module names)
        .init();

    let tank =
        toodeloo_tank::pg::Tank::new("postgres://toodaloo:password@localhost:5432/development_db")
            .await;

    let listener = TcpListener::bind(":::1337").await?;
    println!("\nStarting app on 'http://{}/'\n", listener.local_addr()?);

    let app = create_router(tank).await;

    tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
        4000
    })
    .await?;

    Ok(())
}
