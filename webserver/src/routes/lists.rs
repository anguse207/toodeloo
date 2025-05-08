use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::*, Extension, Json, Router};
use serde::Deserialize;
use toodeloo_core::token::Token;
use toodeloo_tank::pg::Tank;
use tracing::*;

use super::todo_route;

pub fn routes() -> Router<Tank> {
    Router::new()
        .route("/create", post(create))
        .route("/", get(read_all))
        .route("/update/{list_id}", put(todo_route))
        .route("/delete/{list_id}", delete(todo_route))
}

#[derive(Deserialize)]
struct CreateList {
    label: String,
}

async fn create(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
    Json(create_list): Json<CreateList>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Create list - label: {:?}", create_list.label);

    let id = tank.create_list(token.user_id, create_list.label).await.unwrap();

    Ok(Json(id))
}

async fn read_all(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Read lists - User: {:?}", token.user_id);

    let lists = tank.read_lists_from_user_id(token.user_id).await.unwrap();

    Ok(Json(lists))
}