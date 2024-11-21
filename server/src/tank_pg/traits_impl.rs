use toodeloo_core::{
    tank_traits::TaskTank,
    task::{Content, Task},
    user::User,
};

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl TaskTank for Tank {
    // User
    async fn new_user(&self, nick: String) -> Result<Uuid> {
        todo!()
    }
    async fn get_user(&self, id: Uuid) -> Result<User> {
        todo!()
    }
    async fn get_users(&self) -> Result<Vec<User>> {
        todo!()
    }
    async fn update_user(&self, id: Uuid, new: User) -> Result<()> {
        todo!()
    }
    async fn remove_user(&self, id: Uuid) -> Result<()> {
        todo!()
    }

    // List
    async fn new_list(&self, user_id: Uuid, label: &str) -> Result<Uuid> {
        todo!()
    }
    async fn get_list(&self, list_id: Uuid) -> Result<List> {
        todo!()
    }
    async fn get_lists(&self, user_id: Uuid) -> Result<Vec<List>> {
        todo!()
    }
    async fn update_list(&self, list_id: Uuid, new: List) -> Result<()> {
        todo!()
    }
    async fn remove_list(&self, list_id: Uuid) -> Result<()> {
        todo!()
    }

    // Task
    async fn new_task(&self, list_id: Uuid, content: Content) -> Result<Uuid> {
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

