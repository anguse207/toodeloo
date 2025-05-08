use std::time::Duration;

use axum::{extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::*, Json, Router};
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

async fn create(
    State(tank): State<Tank>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let name = headers
        .get("Username")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    let pass = headers
        .get("Password")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    debug!("Create user - username: {:?}", name);

    return match tank.create_user(name, pass).await {
        Ok(user_id) => Ok(Json(user_id)),
        Err(_) => Err(StatusCode::CONFLICT),
    };
}

async fn login(
    State(tank): State<Tank>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let name = headers
        .get("Username")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    let pass = headers
        .get("Password")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    debug!("Login user - username: {:?}", name);

    // Check if the user already exists
    let user = match tank.read_user_by_name(name).await {
        Ok(user) => user,
        Err(_) => return Err((StatusCode::NOT_FOUND, "User doesn't exists")),
    };
    
    // Check if the password is correct
    if user.pass != pass {
        return Err((StatusCode::UNAUTHORIZED, "Wrong password"));
    }

    // Generate token
    let token = tank.create_token(user.id, Duration::from_secs(30)).await.unwrap();

    Ok(Json(token))
}
