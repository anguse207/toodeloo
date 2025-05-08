pub mod lists;
pub mod tasks;
mod token;
pub mod users;

use sqlx::PgPool;

#[derive(Clone)]
pub struct Tank {
    pool: PgPool,
}

impl Tank {
    pub async fn new(connection_str: impl AsRef<str>) -> Self {
        let pool = PgPool::connect(connection_str.as_ref()).await.unwrap();
        Tank { pool }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn pg_test() {
        let tank = Tank::new("postgres://toodaloo:password@localhost:5432/development_db").await;

        let user_id = tank.create_user("Terry", "John").await.unwrap();
        println!("User: {:?}", tank.read_user_by_id(user_id).await.unwrap());

        tank.create_token(user_id, core::time::Duration::from_secs(60))
            .await
            .unwrap();
        tank.create_token(user_id, core::time::Duration::from_secs(60))
            .await
            .unwrap();
        println!(
            "Tokens for user: {:?}",
            tank.read_tokens_from_user_id(user_id).await.unwrap()
        );

        println!("Testing lists...");
        let list_id = tank.create_list(user_id, "My List").await.unwrap();
        println!("List: {:?}", tank.read_list(list_id).await.unwrap());

        tank.create_task(list_id, "My Task", "Do the things")
            .await
            .unwrap();
        tank.create_task(list_id, "My Other Task", "Do the other things")
            .await
            .unwrap();
        println!(
            "Tasks: {:?}",
            tank.read_tasks_from_list_id(list_id).await.unwrap()
        );
    }
}
