use axum::{Router, middleware};
use toodeloo_tank::pg::Tank;
use tower_http::services::ServeDir;
use tracing::*;
use uuid::Uuid;

use crate::auth::auth_middleware;

mod lists;
mod tasks;
mod users;

pub async fn create_router(tank: Tank) -> Router {
    Router::new()
        // Users
        .nest("/api/users", users::routes())
        .nest("/api/lists", lists::routes())
        .nest("/api/tasks", tasks::routes())
        // Lists
        // Tasks
        // State
        .with_state(tank.clone())
        // Middleware
        .layer(middleware::from_fn_with_state(
            tank.clone(),
            auth_middleware,
        ))
        // Serve react app
        .fallback_service(ServeDir::new("frontend-react/dist"))
}

pub async fn todo_route() {
    error!("!! HIT TODO ROUTE !!");
}

pub struct UuidWrapper {
    pub id: Uuid,
}