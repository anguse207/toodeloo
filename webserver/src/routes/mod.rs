use axum::{
    Router, middleware,
    response::IntoResponse,
};
use toodeloo_tank::sqlite::Tank;
use tower_http::services::ServeDir;
use tracing::info;

use crate::auth::auth_middleware;

mod users;
mod lists;
mod tasks;

pub async fn create_router(tank: Tank) -> Router {
    Router::new()
        // Users
        .nest("/users", users::routes())
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
    info!("Todo route");
}