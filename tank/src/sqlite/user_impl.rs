use toodeloo_core::{
    DEFAULT_DELETED_TIME,
    user::{UpdateUser, User},
};

use anyhow::Result;
use sqlx::{query, query_as};
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn new_user(&self, name: impl AsRef<str>, pass: impl AsRef<str>) -> Result<Uuid> {
        let id = Uuid::new_v4();

        // Insert the new user
        query("INSERT INTO users (id, name, pass, deleted_time) VALUES (?, ?, ?, ?)")
            .bind(id.to_string())
            .bind(name.as_ref())
            .bind(pass.as_ref())
            .bind(DEFAULT_DELETED_TIME)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    pub async fn get_user(&self, id: &Uuid) -> Result<User> {
        let user =
            query_as::<_, User>("SELECT id, name, pass, deleted_time FROM users WHERE id = ?")
                .bind(id.to_string())
                .fetch_one(&self.pool)
                .await?;

        Ok(user)
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let users = query_as::<_, User>("SELECT * FROM users WHERE deleted_time = 0")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn update_user(&self, id: &Uuid, new: &UpdateUser) -> Result<()> {
        let _ = query("UPDATE users SET name = ?, pass = ?, deleted_time = ? WHERE id = ?")
            .bind(&new.name)
            .bind(&new.pass)
            .bind(new.deleted_time as i64)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // This would only be used in batch, soft delete is done in `update_user()``
    // This will fail if there is a foreign key constraint (lists, tasks)
    pub async fn remove_user(&self, id: &Uuid) -> Result<()> {
        let _ = query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
