use anyhow::Result;
use uuid::Uuid;

use crate::{
    list::List,
    task::Task,
    user::{UpdateUser, User},
};

// Traits for implementing a storage mechanism
#[async_trait::async_trait]
pub trait UserTank {
    // User
    async fn new_user(&self, nick: &str) -> Result<Uuid>;
    async fn get_user(&self, id: &Uuid) -> Result<User>;
    async fn get_users(&self) -> Result<Vec<User>>;
    async fn update_user(&self, id: &Uuid, new: &UpdateUser) -> Result<()>;
    async fn remove_user(&self, id: &Uuid) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ListTank {
    // List
    async fn new_list(&self, user_id: &Uuid, label: &str) -> Result<Uuid>;
    async fn get_list(&self, list_id: &Uuid) -> Result<List>;
    async fn get_lists(&self, user_id: &Uuid) -> Result<Vec<List>>;
    async fn update_list(&self, list_id: &Uuid, new: &List) -> Result<()>;
    async fn remove_list(&self, list_id: &Uuid) -> Result<()>;
}

#[async_trait::async_trait]
pub trait TaskTank {
    // Task
    async fn new_task(&self, list_id: &Uuid, title: &str, content: &str) -> Result<Uuid>;
    async fn get_task(&self, id: &Uuid) -> Result<Task>;
    async fn get_tasks(&self, list_id: &Uuid) -> Result<Vec<Task>>;
    async fn update_task(&self, id: &Uuid, new: &Task) -> Result<()>;
    async fn remove_task(&self, id: &Uuid) -> Result<()>;
}
