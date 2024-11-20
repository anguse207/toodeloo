mod traits_impl;

use anyhow::Result;
use deadpool_postgres::{tokio_postgres::NoTls, Config, Pool, Runtime};

#[derive(Debug)]
pub struct Tank {
    pool: Pool,
}

impl Tank {
    pub async fn new() -> Result<Self> {
        let mut cfg = Config::new();
        cfg.user = Some(String::from("postgres"));
        cfg.password = Some(String::from("postgres"));
        cfg.host = Some(String::from("127.0.0.1"));
        cfg.port = Some(5435);
        cfg.dbname = Some(String::from("postgres"));
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

        Ok(Self { pool })
    }

    pub async fn new_from_pool(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create_tables(&self) -> Result<()> {
        todo!();
        Ok(())
    }
}
