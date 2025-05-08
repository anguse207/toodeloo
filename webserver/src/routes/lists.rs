use axum::{Router, routing::*};
use toodeloo_tank::pg::Tank;

use super::todo_route;

pub fn routes() -> Router<Tank> {
    Router::new()
        .route("/create", post(todo_route))
        .route("/", get(todo_route))
        .route("/update/{list_id}", put(todo_route))
        .route("/delete/{list_id}", delete(todo_route))
}
