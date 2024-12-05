use toodeloo_core::{
    tank_traits::UserTank,
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
    async fn new_user(&self, nick: &str) -> Result<Uuid> {
        // TODO: Make sure the id is unique
        let id = Uuid::new_v4();
        
        // Insert the new user
        query("INSERT INTO users (id, nick, token, deleted_time) VALUES (?, ?, ?, ?)")
            .bind(&id.to_string())
            .bind(nick)
            .bind(Uuid::new_v4().to_string())
            .bind(0)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    async fn get_user(&self, id: &Uuid) -> Result<User> {
        let user = query_as::<_, User>("SELECT id, nick, token, deleted_time FROM users WHERE id = ?")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_users(&self) -> Result<Vec<User>> {
        let users = query_as::<_, User>("SELECT * FROM users WHERE deleted_time = 0")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn update_user(&self, id: &Uuid, new: &User) -> Result<()> {
        query("UPDATE users SET nick = ?, deleted_time = ?, token = ? WHERE id = ?")
            .bind(&new.nick)
            .bind(new.deleted_time as i64)
            .bind(new.token)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // This would only be used in batch, soft delete is done in `update_user()``
    async fn remove_user(&self, id: &Uuid) -> Result<()> {
        let rows = query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
