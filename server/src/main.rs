mod routes;
mod tank_sqlite;

use std::fs::{self, File};
use std::sync::Arc;

use anyhow::{Ok, Result};
use axum::Router;
use routes::create_router;
use tank_sqlite::init_db;
use tank_sqlite::Tank;
use toodeloo_core::tank_traits::*;

use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // setup tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Set max level to DEBUG
        .init();
    info!("Starting Logging...");

    let tank = init_db("testing.db").await?;

    let app = create_router(tank).await;

    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
