use anyhow::Result;
use toodeloo_core::task::Task;
use uuid::Uuid;

use super::Tank;

impl Tank {
    pub async fn create_task(
        &self,
        list_id: Uuid,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<Uuid> {
        let task = Task::new(list_id, title.into(), content.into());
        let query = sqlx::query!(
            "INSERT INTO tasks (id, list_id, origin_time, title, content, done, snoozed_until, deleted_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            task.id,
            task.list_id,
            task.origin_time,
            task.title,
            task.content,
            task.done,
            task.snoozed_until,
            task.deleted_time
        ).execute(&self.pool)
        .await;

        match query {
            Ok(_) => {
                println!("Task created: {:?}", task);
                Ok(task.id)
            }
            Err(e) => {
                eprintln!("Error creating Task: {:?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn read_task(&self, id: Uuid) -> Result<Task> {
        let query = sqlx::query_as!(
            Task,
            "SELECT id, list_id, origin_time, title, content, done, snoozed_until, deleted_time FROM tasks WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(query)
    }

    pub async fn read_tasks_from_list_id(&self, id: Uuid) -> Result<Vec<Task>> {
        let query = sqlx::query_as!(
            Task,
            "SELECT id, list_id, origin_time, title, content, done, snoozed_until, deleted_time FROM tasks WHERE list_id = $1",
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query)
    }

    pub async fn update_task(&self, task: &Task) -> Result<Task> {
        let updated_task = sqlx::query_as!(
            Task,
            "UPDATE tasks SET list_id = $1, origin_time = $2, title = $3, content = $4, done = $5, snoozed_until = $6, deleted_time = $7 WHERE id = $8 RETURNING id, list_id, origin_time, title, content, done, snoozed_until, deleted_time",
            task.list_id,
            task.origin_time,
            task.title,
            task.content,
            task.done,
            task.snoozed_until,
            task.deleted_time,
            task.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_task)
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
