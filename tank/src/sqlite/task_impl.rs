use anyhow::Result;
use async_trait::async_trait;
use sqlx::{query, query_as};
use toodeloo_core::{
    DEFAULT_DELETED_TIME, DEFAULT_SNOOZED_UNTIL, task::Task, timing::get_timestamp,
};
use uuid::Uuid;

use super::Tank;

impl Tank {
    // Task
    pub async fn new_task(&self, list_id: &Uuid, title: &str, content: &str) -> Result<Uuid> {
        let id = Uuid::new_v4();

        query("INSERT INTO tasks (id, list_id, origin_time, title, content, done, snoozed_until, deleted_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(id.to_string())
            .bind(list_id.to_string())
            .bind(get_timestamp() as i64)
            .bind(title)
            .bind(content)
            .bind(false)
            .bind(DEFAULT_SNOOZED_UNTIL)
            .bind(DEFAULT_DELETED_TIME)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    pub async fn get_task(&self, id: &Uuid) -> Result<Task> {
        let task = query_as::<_, Task>("SELECT id, list_id, origin_time, title, content, done, snoozed_until, deleted_time FROM tasks WHERE id = ?")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await?;

        Ok(task)
    }

    pub async fn get_tasks(&self, list_id: &Uuid) -> Result<Vec<Task>> {
        let tasks =
            query_as::<_, Task>("SELECT * FROM tasks WHERE list_id = ? AND deleted_time = 0")
                .bind(list_id.to_string())
                .fetch_all(&self.pool)
                .await?;

        Ok(tasks)
    }

    pub async fn update_task(&self, id: &Uuid, new: &Task) -> Result<()> {
        query("UPDATE tasks SET list_id = ?, origin_time = ?, title = ?, content = ?, done = ?, snoozed_until = ?, deleted_time = ? WHERE id = ?")
            .bind(new.list_id.to_string())
            .bind(new.origin_time as i64)
            .bind(new.title.to_string())
            .bind(new.content.to_string())
            .bind(new.done)
            .bind(new.snoozed_until as i64)
            .bind(new.deleted_time as i64)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn remove_task(&self, id: &Uuid) -> Result<()> {
        let _ = query("DELETE FROM tasks WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
