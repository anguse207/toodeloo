use auth::auth_middleware;
use axum::{Router, middleware};
use toodeloo_tank::pg::Tank;
use tower_http::services::{ServeDir, ServeFile};
use tracing::*;

mod auth;
mod lists;
mod tasks;
mod users;

pub type RouterType = Router<Tank>;

pub async fn create_router(tank: Tank) -> Router {
    Router::new()
        // Users
        .nest("/api/users", users::routes())
        .nest("/api/lists", lists::routes())
        .nest("/api/tasks", tasks::routes())
        .nest("/auth/discord", auth::discord::routes())
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
        .fallback_service(ServeDir::new("frontend-react/dist")
            .fallback(ServeFile::new("frontend-react/dist/index.html"))
        )
}

pub async fn todo_route() {
    error!("!! HIT TODO ROUTE !!");
}
