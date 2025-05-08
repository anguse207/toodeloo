use std::time::Duration;

use axum::{
    extract::{Path, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::*, Json, Router
};
use sqlx::{query, query_as};
// use serde::Deserialize;
use toodeloo_core::{token::Token, user::{UpdateUser, User}, DEFAULT_DELETED_TIME};
use toodeloo_tank::sqlite::Tank;
use tracing::*;
use uuid::Uuid;

use super::todo_route;

pub fn routes() -> Router<Tank> {
    Router::new()
        .route("/create", post(create))
        .route("/login", post(login))
        .route("/",get(todo_route))
        .route("/update",put(todo_route))
        .route("/delete",delete(todo_route))
}

async fn create(
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

    debug!("Create user - username: {:?}", name);

    let id = Uuid::new_v4();

    // Check if the user already exists
    let user = query("SELECT id FROM users WHERE name = ?")
        .bind(name)
        .fetch_optional(&tank.pool)
        .await.unwrap();

    if user.is_some() {
        return Err((StatusCode::CONFLICT, "User already exists"));
    }

    // Insert the new user
    query("INSERT INTO users (id, name, pass, deleted_time) VALUES (?, ?, ?, ?)")
        .bind(id.to_string())
        .bind(name)
        .bind(pass)
        .bind(DEFAULT_DELETED_TIME)
        .execute(&tank.pool)
        .await.unwrap();

    Ok(StatusCode::CREATED)
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
    let user = query_as::<_, User>("SELECT id FROM Userss WHERE name = ?")
        .bind(name)
        .fetch_optional(&tank.pool)
        .await.unwrap();
    
    if user.is_none() {
        return Err((StatusCode::CONFLICT, "User doesn't exists"));
    }

    // Check if the password is correct
    let user = user.unwrap();
    if user.pass != pass {
        return Err((StatusCode::UNAUTHORIZED, "Wrong password"));
    }
    
    // Create token
    let token = Token::new(user.id, Duration::from_secs(60 * 60 * 24));

    // Insert the token into the database
    query("INSERT INTO tokens (id, user_id, token) VALUES (?, ?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(user.id.to_string())
        .bind(token.expiry as i64)
        .bind(false)
        .execute(&tank.pool)
        .await.unwrap();
    
    Ok(Json(token))
}

pub async fn fetch(State(tank): State<Tank>) -> impl IntoResponse {
    debug!("GET USER - User ID: {:?}", 0);

    // if let Ok(user_id) = user_id.parse::<Uuid>() {
    //     let user = tank.get_user(&user_id).await;

    //     match user {
    //         Ok(user) => (StatusCode::OK, Json(Some(user))),
    //         Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    //     }
    // } else {
    //     (StatusCode::BAD_REQUEST, Json(None))
    // }
}

pub async fn fetch_all(State(tank): State<Tank>) -> impl IntoResponse {
    debug!("GET USERS");

    let users = tank.get_users().await.unwrap();
    Json(users)
}

pub async fn update(
    State(tank): State<Tank>,
    // Path(user_id): Path<String>,
    Json(update_user): Json<UpdateUser>,
) -> impl IntoResponse {
    debug!(
        "PUT USER - User ID: {:?}, UpdateUser: {:?}",
        0, update_user
    );

    // tank.update_user(.parse().unwrap(), &update_user)
    //     .await
    //     .unwrap()
}

pub async fn remove_user(
    State(tank): State<Tank>,
) -> impl IntoResponse {
    debug!("DELETE USER - User ID: {:?}", 0);

    // tank.remove_user(&user_id.parse().unwrap()).await.unwrap()
}
