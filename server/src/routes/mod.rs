use axum::{
    Router,
    routing::{get, post},
};
use toodeloo_tank::sqlite::Tank;
use tracing::info;


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
