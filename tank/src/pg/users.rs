use anyhow::Result;
use toodeloo_core::user::User;
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn create_user(&self, name: impl AsRef<str>, pass: impl AsRef<str>) -> Result<Uuid> {
        let user = User::new(name.as_ref(), pass.as_ref());

        let query = sqlx::query!(
            "INSERT INTO users (id, name, pass, deleted_time) VALUES ($1, $2, $3, $4)",
            user.id,
            user.name,
            user.pass,
            user.deleted_time
        )
        .execute(&self.pool)
        .await;

        match query {
            Ok(_) => {
                println!("User created: {:?}", user);
                Ok(user.id)
            }
            Err(e) => {
                eprintln!("Error creating user: {:?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn read_user_by_id(&self, id: Uuid) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, pass, deleted_time FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn read_user_by_name(&self, name: impl AsRef<str>) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, pass, deleted_time FROM users WHERE name = $1",
            name.as_ref()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        let updated_user = sqlx::query_as!(
            User,
            "UPDATE users SET name = $1, pass = $2, deleted_time = $3 WHERE id = $4 RETURNING id, name, pass, deleted_time",
            user.name,
            user.pass,
            user.deleted_time,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
