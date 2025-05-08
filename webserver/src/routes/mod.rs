use axum::{Router, middleware};
use toodeloo_tank::pg::Tank;
use tower_http::services::ServeDir;
use tracing::*;

use crate::auth::auth_middleware;

mod lists;
mod tasks;
mod users;

pub async fn create_router(tank: Tank) -> Router {
    Router::new()
        // Users
        .nest("/users", users::routes())
        .nest("/lists", lists::routes())
        .nest("/tasks", tasks::routes())
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
        .fallback_service(ServeDir::new("frontend"))
}

pub async fn todo_route() {
    error!("!! HIT TODO ROUTE !!");
}
