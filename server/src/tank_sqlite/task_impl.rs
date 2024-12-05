use toodeloo_core::{
    tank_traits::TaskTank,
    task::{Content, Task},
};

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl TaskTank for Tank {
    // Task
    async fn new_task(&self, list_id: Uuid, title: String, content: Content) -> Result<Uuid> {
        todo!()
    }
    async fn get_task(&self, id: Uuid) -> Result<Task> {
        todo!()
    }
    async fn get_tasks(&self, list_id: Uuid) -> Result<Vec<Task>> {
        todo!()
    }
    async fn update_task(&self, id: Uuid, new: Task) -> Result<()> {
        todo!()
    }
    async fn remove_task(&self, id: Uuid) -> Result<()> {
        todo!()
    }
}
