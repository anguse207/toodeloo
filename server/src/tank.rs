
use toodeloo_core::{tank_traits::TaskTank, task::Task, user::User};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Tank {
    pool: SqlitePool,
}

impl Tank {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskTank for Tank {
    async fn new_user(&self, nick: String) -> Result<Uuid> {
        todo!()
    }
    async fn new_task(&self, user_id: Uuid, content: String) -> Result<Uuid> {
        todo!()
    }
    async fn get_user(&self, id: Uuid) -> Result<User> {
        todo!()
    }
    async fn get_task(&self, task_id: Uuid) -> Result<Task> {
        todo!()
    }
    async fn get_users(&self) -> Result<Vec<User>> {
        todo!()
    }
    async fn get_tasks(&self, user_id: Uuid) -> Result<Vec<Task>> {
        todo!()
    }
    async fn update_task(&self, task_id: Uuid, new_task: Task) -> Result<()> {
        todo!()
    }
    async fn remove_task(&self, task_id: Uuid) -> Result<()> {
        todo!()
    }
}
