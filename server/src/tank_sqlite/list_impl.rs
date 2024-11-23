use toodeloo_core::{
    list::List,
    tank_traits::{ListTank, ID},
};

use anyhow::Result;
use async_trait::async_trait;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl ListTank for Tank {
    // List
    async fn new_list(&self, user_id: ID, label: &str) -> Result<ID> {
        todo!()
    }
    async fn get_list(&self, list_id: ID) -> Result<List> {
        todo!()
    }
    async fn get_lists(&self, user_id: ID) -> Result<Vec<List>> {
        todo!()
    }
    async fn update_list(&self, list_id: ID, new: List) -> Result<()> {
        todo!()
    }
    async fn remove_list(&self, list_id: ID) -> Result<()> {
        todo!()
    }
}
