use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::*,
};
use serde::Deserialize;
use toodeloo_core::token::Token;
use toodeloo_tank::pg::Tank;
use tracing::*;
use uuid::Uuid;

use super::{RouterType, todo_route};

pub fn routes() -> RouterType {
    Router::new()
        .route("/create", post(create))
        .route("/", get(read_all))
        .route("/{list_id}", get(read))
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
    Json(CreateList { label }): Json<CreateList>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Create list - label: {:?}", label);

    let id = tank.create_list(token.user_id, label).await.unwrap();

    Ok(Json(id))
}

async fn read(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
    Path(list_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Read list - User: {:?}", token.user_id);

    let list = tank.read_list(list_id).await.unwrap();

    if list.user_id != token.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            "You are not allowed to read this list",
        ));
    }

    Ok(Json(list))
}

async fn read_all(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Read lists - User: {:?}", token.user_id);

    let lists = tank.read_lists_from_user_id(token.user_id).await.unwrap();

    Ok(Json(lists))
}
