pub mod lists;
pub mod tasks;
mod token;
pub mod users;

use sqlx::PgPool;

#[derive(Clone)]
pub struct Tank {
    pool: PgPool,
}

impl Tank {
    pub fn new(connection_str: impl AsRef<str>) -> Self {
        let pool = PgPool::connect_lazy(connection_str.as_ref()).unwrap();
        Tank { pool }
    }
}
