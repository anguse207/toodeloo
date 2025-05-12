use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::*, Extension, Json, Router};
use serde::Deserialize;
use toodeloo_core::{task, token::Token};
use toodeloo_tank::pg::Tank;
use tracing::*;
use uuid::Uuid;

use super::{todo_route, RouterType};

pub fn routes() -> RouterType {
    Router::new()
        .route("/create", post(create))
        .route("/{list_id}", get(read_all))
        .route("/{list_id}/{task_id}", get(read))
        .route("/update/{task_id}", put(todo_route))
        .route("/delete/{task_id}", delete(todo_route))
}

#[derive(Deserialize)]
struct CreateTask {
    list_id: Uuid,
    title: String,
    content: String,
}

async fn create(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
    Json(CreateTask { list_id, title, content }): Json<CreateTask>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Create task - User: {:?}", token.user_id);

    let list = tank.read_list(list_id).await.unwrap();

    if list.user_id != token.user_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to edit this list"));
    }

    let id = tank.create_task(list_id, title, content).await.unwrap();

    Ok(Json(id))
}

async fn read(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
    Path((list_id,task_id)): Path<(Uuid,Uuid)>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Read task - User: {:?}", token.user_id);

    let task = tank.read_task(task_id).await.unwrap();
    let list = tank.read_list(list_id).await.unwrap();

    if list.user_id != token.user_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to read this list"));
    }

    if task.list_id != list_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to read this task"));
    }

    Ok(Json(task))
}

async fn read_all(
    State(tank): State<Tank>,
    Extension(token): Extension<Token>,
    Path(list_id): Path<Uuid>
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Read tasks - User: {:?}", token.user_id);

    let list = tank.read_list(list_id).await.unwrap();

    if list.user_id != token.user_id {
        return Err((StatusCode::FORBIDDEN, "You are not allowed to read this list"));
    }

    let tasks = tank.read_tasks_from_list_id(list_id).await.unwrap();

    Ok(Json(tasks))
}