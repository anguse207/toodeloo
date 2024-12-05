use toodeloo_core::{
    list::List,
    tank_traits::ListTank,
};

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl ListTank for Tank {
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
}
