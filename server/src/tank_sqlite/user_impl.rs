use toodeloo_core::{tank_traits::UserTank, user::User};

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl UserTank for Tank {
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
}
