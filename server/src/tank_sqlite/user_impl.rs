use toodeloo_core::{
    tank_traits::{UserTank, ID},
    user::User,
};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use super::Tank;

#[allow(unused)]
#[async_trait]
impl UserTank for Tank {
    async fn new_user(&self, nick: String) -> Result<ID> {
        // Make sure the id is unique
        let mut id: Uuid = Uuid::new_v4();
        while (self.get_user(id.to_string()).await.is_ok()) {
            id = Uuid::new_v4();
        }

        query("INSERT INTO users (id, nick, deleted_time) VALUES (?, ?, ?)")
            .bind(id.clone().to_string())
            .bind(nick)
            .bind(0)
            .execute(&self.pool)
            .await?;

        Ok(id.to_string())
    }

    async fn get_user(&self, id: ID) -> Result<User> {
        let user = query_as::<_, User>("SELECT id, nick, deleted_time FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_users(&self) -> Result<Vec<User>> {
        let users = query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn update_user(&self, id: ID, new: User) -> Result<()> {
        query("UPDATE users SET nick = ?, deleted_time = ? WHERE id = ?")
            .bind(new.nick)
            .bind(new.deleted_time as i64)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // This would only be used in batch, soft delete is done in `update_user()``
    async fn remove_user(&self, id: ID) -> Result<()> {
        query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
