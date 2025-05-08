use axum::{routing::*, Router};
use toodeloo_tank::sqlite::Tank;

use super::todo_route;

pub fn routes() -> Router<Tank> {
    Router::new()
        .route("/create", post(todo_route))
        .route("/{list_id}",get(todo_route))
        .route("/update/{task_id}",put(todo_route))
        .route("/delete/{task_id}",delete(todo_route))
}