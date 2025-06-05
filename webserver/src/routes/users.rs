use std::time::Duration;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::*,
};
use serde::Deserialize;
use toodeloo_core::{timing::get_timestamp, token::Token};
use toodeloo_tank::pg::Tank;
use tracing::*;

use super::{RouterType, todo_route};

pub fn routes() -> RouterType {
    Router::new()
        // .route("/create", post(create))
        // .route("/login", post(login))
        .route("/", get(read))
        .route("/update", put(todo_route))
        .route("/delete", delete(soft_delete))
}

async fn read(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Read list - User: {:?}", token.user_id);

    let user = tank.read_user_by_id(token.user_id).await.unwrap();

    Ok(Json(user))
}

async fn soft_delete(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Soft delete user - User: {:?}", token.user_id);

    let mut user = tank.read_user_by_id(token.user_id).await.unwrap();

    user.deleted_time = get_timestamp();

    tank.update_user(&user).await.unwrap();

    // TODO: Revoke token?
    Ok((StatusCode::OK, "User deleted"))
}
