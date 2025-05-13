use anyhow::Result;
use toodeloo_core::user::{OauthProvider, User};
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn create_user(
        &self,
        oauth_id: impl Into<String>,
        oauth_provider: OauthProvider,
        nickname: impl Into<String>,
    ) -> Result<Uuid> {
        let user = User::new(oauth_id, oauth_provider, nickname);

        let query = sqlx::query!(
            r#"
            INSERT INTO users (id, oauth_id, oauth_provider, nickname, deleted_time)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.oauth_id,
            user.oauth_provider as OauthProvider, // Cast the enum
            user.nickname,
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
            r#"
            SELECT id, oauth_id, oauth_provider as "oauth_provider: OauthProvider", nickname, deleted_time
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn read_user_by_oauth_id(&self, oauth_id: impl Into<String>) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, oauth_id, oauth_provider as "oauth_provider: OauthProvider", nickname, deleted_time
            FROM users
            WHERE oauth_id = $1
            "#,
            oauth_id.into()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // pub async fn read_user_by_nickname(&self, nickname: impl AsRef<str>) -> Result<User> {
    //     let user = sqlx::query_as!(
    //         User,
    //         r#"
    //         SELECT id, oauth_id, oauth_provider as "oauth_provider: OauthProvider", nickname, deleted_time
    //         FROM users
    //         WHERE nickname = $1
    //         "#,
    //         nickname.as_ref()
    //     )
    //     .fetch_one(&self.pool)
    //     .await?;

    //     Ok(user)
    // }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET oauth_id = $1, oauth_provider = $2, nickname = $3, deleted_time = $4
            WHERE id = $5
            RETURNING id, oauth_id, oauth_provider as "oauth_provider: OauthProvider", nickname, deleted_time
            "#,
            user.oauth_id,
            user.oauth_provider as OauthProvider, // Cast the enum
            user.nickname,
            user.deleted_time,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
