use std::time::Duration;

use anyhow::Result;
use toodeloo_core::token::Token;
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn create_token(&self, user_id: Uuid, duration: Duration) -> Result<Token> {
        let token = Token::new(user_id, duration);

        let query = sqlx::query!(
            "INSERT INTO tokens (id, user_id, expiry, revoked) VALUES ($1, $2, $3, $4)",
            token.id,
            token.user_id,
            token.expiry,
            token.revoked
        )
        .execute(&self.pool)
        .await;

        match query {
            Ok(_) => {
                println!("Token created: {:?}", token);
                Ok(token)
            }
            Err(e) => {
                eprintln!("Error creating token: {:?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn read_tokens_from_user_id(&self, user_id: Uuid) -> Result<Vec<Token>> {
        let query = sqlx::query_as!(
            Token,
            "SELECT id, user_id, expiry, revoked FROM tokens WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query)
    }

    pub async fn read_token(&self, id: Uuid) -> Result<Token> {
        let query = sqlx::query_as!(
            Token,
            "SELECT id, user_id, expiry, revoked FROM tokens WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(query)
    }

    pub async fn update_token(&self, token: &Token) -> Result<Token> {
        let updated_token = sqlx::query_as!(
            Token,
            "UPDATE tokens SET user_id = $1, expiry = $2, revoked = $3 WHERE id = $4 RETURNING id, user_id, expiry, revoked",
            token.user_id,
            token.expiry,
            token.revoked,
            token.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_token)
    }

    pub async fn delete_token(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM tokens WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
