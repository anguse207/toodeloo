use anyhow::Result;

use crate::{
    list::List,
    task::{Content, Task},
    user::User,
};

pub type ID = String;

// Traits for implementing a storage mechanism
#[async_trait::async_trait]
pub trait UserTank {
    // User
    async fn new_user(&self, nick: String) -> Result<ID>;
    async fn get_user(&self, id: ID) -> Result<User>;
    async fn get_users(&self) -> Result<Vec<User>>;
    async fn update_user(&self, id: ID, new: User) -> Result<()>;
    async fn remove_user(&self, id: ID) -> Result<()>;
}

// Traits for implementing a storage mechanism
#[async_trait::async_trait]
pub trait ListTank {
    // List
    async fn new_list(&self, user_id: ID, label: &str) -> Result<ID>;
    async fn get_list(&self, list_id: ID) -> Result<List>;
    async fn get_lists(&self, user_id: ID) -> Result<Vec<List>>;
    async fn update_list(&self, list_id: ID, new: List) -> Result<()>;
    async fn remove_list(&self, list_id: ID) -> Result<()>;
}

// Traits for implementing a storage mechanism
#[async_trait::async_trait]
pub trait TaskTank {
    // Task
    async fn new_task(&self, list_id: ID, content: Content) -> Result<ID>;
    async fn get_task(&self, id: ID) -> Result<Task>;
    async fn get_tasks(&self, list_id: ID) -> Result<Vec<Task>>;
    async fn update_task(&self, id: ID, new: Task) -> Result<()>;
    async fn remove_task(&self, id: ID) -> Result<()>;
}
