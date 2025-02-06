use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;

use crate::tank_sqlite::Tank;

mod users;

pub async fn create_router(tank: Tank) -> Router {
    Router::new()
        .route("/", get(test_handler))
        // Users
        .route("/users", get(users::get_users))
        .route(
            "/users/:param",
            post(users::new_user)
                .get(users::get_user)
                .put(users::update_user)
                .delete(users::remove_user),
        )
        // Lists
        // Tasks
        .with_state(tank)
}

async fn test_handler() -> impl axum::response::IntoResponse {
    info!("Test Handler");
    "Hello, World!"
}
