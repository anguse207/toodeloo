use uuid::Uuid;
use anyhow::Result;

use crate::{task::Task, user::User};

#[async_trait::async_trait]
pub trait TaskTank {
    async fn new_user(&self, nick: String) -> Result<Uuid>;
    async fn new_task(&self, user_id: Uuid, content: String) -> Result<Uuid>;

    async fn get_user(&self, id: Uuid) -> Result<User>;
    async fn get_task(&self, task_id: Uuid) -> Result<Task>;
    
    async fn get_users(&self) -> Result<Vec<User>>;
    async fn get_tasks(&self, user_id: Uuid) -> Result<Vec<Task>>;

    async fn update_task(&self, task_id: Uuid, new_task: Task) -> Result<()>;
    async fn remove_task(&self, task_id: Uuid) -> Result<()>;
}