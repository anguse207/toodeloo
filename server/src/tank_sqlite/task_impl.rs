use toodeloo_core::{
    tank_traits::{TaskTank, ID},
    task::{Content, Task},
};

use anyhow::Result;
use async_trait::async_trait;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl TaskTank for Tank {
    // Task
    async fn new_task(&self, list_id: ID, content: Content) -> Result<ID> {
        todo!()
    }
    async fn get_task(&self, id: ID) -> Result<Task> {
        todo!()
    }
    async fn get_tasks(&self, list_id: ID) -> Result<Vec<Task>> {
        todo!()
    }
    async fn update_task(&self, id: ID, new: Task) -> Result<()> {
        todo!()
    }
    async fn remove_task(&self, id: ID) -> Result<()> {
        todo!()
    }
}
