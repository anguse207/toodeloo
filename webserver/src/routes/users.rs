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
        .route("/", get(todo_route))
        .route("/update", put(todo_route))
        .route("/delete", delete(soft_delete))
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

// async fn create(
//     State(tank): State<Tank>,
//     Json(Credentials { username, password }): Json<Credentials>,
// ) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
//     debug!("Create user - username: {:?}", username);

//     return match tank.create_user(username, password).await {
//         Ok(user_id) => Ok(Json(user_id)),
//         Err(_) => Err((StatusCode::CONFLICT, "User creation failed")),
//     };
// }

// async fn login(
//     State(tank): State<Tank>,
//     Json(Credentials { username, password }): Json<Credentials>,
// ) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
//     debug!("Login user - username: {:?}", username);

//     // Check if the user already exists
//     let user = match tank.read_user_by_name(username).await {
//         Ok(user) => user,
//         Err(_) => return Err((StatusCode::NOT_FOUND, "User doesn't exists")),
//     };

//     // Check if the password is correct
//     if user.pass != password {
//         return Err((StatusCode::UNAUTHORIZED, "Wrong password"));
//     }

//     // Generate token
//     let token_id = tank
//         .create_token(user.id, Duration::from_secs(30))
//         .await
//         .unwrap();
//     let token = tank.read_token(token_id).await.unwrap();
//     Ok(Json(token))
// }

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
