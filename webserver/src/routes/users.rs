use std::time::Duration;

use axum::{extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::*, Json, Router};
use serde::Deserialize;
use toodeloo_tank::pg::Tank;
use tracing::*;

use super::todo_route;

pub fn routes() -> Router<Tank> {
    Router::new()
        .route("/create", post(create))
        .route("/login", post(login))
        .route("/", get(todo_route))
        .route("/update", put(todo_route))
        .route("/delete", delete(todo_route))
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

async fn create(
    State(tank): State<Tank>,
    Json(Credentials { username, password }): Json<Credentials>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Create user - username: {:?}", username);

    return match tank.create_user(username, password).await {
        Ok(user_id) => Ok(Json(user_id)),
        Err(_) => Err((StatusCode::CONFLICT, "User creation failed")),
    };
}

async fn login(
    State(tank): State<Tank>,
    Json(Credentials { username, password }): Json<Credentials>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    debug!("Login user - username: {:?}", username);

    // Check if the user already exists
    let user = match tank.read_user_by_name(username).await {
        Ok(user) => user,
        Err(_) => return Err((StatusCode::NOT_FOUND, "User doesn't exists")),
    };
    
    // Check if the password is correct
    if user.pass != password {
        return Err((StatusCode::UNAUTHORIZED, "Wrong password"));
    }

    // Generate token
    let token_id = tank.create_token(user.id, Duration::from_secs(30)).await.unwrap();
    let token = tank.read_token(token_id).await.unwrap();
    Ok(Json(token))
}
