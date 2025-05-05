use sqlx::{query, query_as};
use toodeloo_core::list::List;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn new_list(&self, user_id: &Uuid, label: &str) -> Result<Uuid> {
        let id = Uuid::new_v4();

        query("INSERT INTO lists (id, user_id, label, deleted_time) VALUES (?, ?, ?, ?)")
            .bind(id.to_string())
            .bind(user_id.to_string())
            .bind(label)
            .bind(0)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    pub async fn get_list(&self, list_id: &Uuid) -> Result<List> {
        let list =
            query_as::<_, List>("SELECT id, user_id, label, deleted_time FROM lists WHERE id = ?")
                .bind(list_id.to_string())
                .fetch_one(&self.pool)
                .await?;

        Ok(list)
    }

    pub async fn get_lists(&self, user_id: &Uuid) -> Result<Vec<List>> {
        let lists =
            query_as::<_, List>("SELECT * FROM lists WHERE user_id = ? AND deleted_time = 0")
                .bind(user_id.to_string())
                .fetch_all(&self.pool)
                .await?;

        Ok(lists)
    }

    pub async fn update_list(&self, list_id: &Uuid, new: &List) -> Result<()> {
        query("UPDATE lists SET user_id = ?, label = ?, deleted_time = ? WHERE id = ?")
            .bind(new.user_id.to_string())
            .bind(new.label.to_string())
            .bind(new.deleted_time as i64)
            .bind(list_id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn remove_list(&self, list_id: &Uuid) -> Result<()> {
        let _ = query("DELETE FROM lists WHERE id = ?")
            .bind(list_id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
