use anyhow::Result;
use toodeloo_core::{list::List};
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn create_list(&self, user_id: Uuid, label: impl AsRef<str>) -> Result<Uuid> {
        let list = List::new(user_id, label.as_ref());
        let query = sqlx::query!(
            "INSERT INTO lists (id, user_id, label, deleted_time) VALUES ($1, $2, $3, $4)",
            list.id,
            list.user_id,
            list.label,
            list.deleted_time
        )
        .execute(&self.pool)
        .await;

        match query {
            Ok(_) => {
                println!("List created: {:?}", list);
                Ok(list.id)
            }
            Err(e) => {
                eprintln!("Error creating List: {:?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn read_list(&self, id: Uuid) -> Result<List> {
        let query = sqlx::query_as!(
            List,
            "SELECT id, user_id, label, deleted_time FROM lists WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(query)
    }

    pub async fn update_list(&self, list: &List) -> Result<List> {
        let updated_list = sqlx::query_as!(
            List,
            "UPDATE lists SET user_id = $1, label = $2, deleted_time = $3 WHERE id = $4 RETURNING id, user_id, label, deleted_time",
            list.user_id,
            list.label,
            list.deleted_time,
            list.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_list)
    }

    pub async fn delete_list(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM lists WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
