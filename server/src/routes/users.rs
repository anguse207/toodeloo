use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use toodeloo_core::{tank_traits::UserTank, user::UpdateUser};
use tracing::*;
use uuid::Uuid;

use crate::tank_sqlite::Tank;

pub async fn new_user(
    State(tank): State<Tank>,
    Path(nickname): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    debug!("GET USER - User ID: {:?}", nickname);

    let auth_token = match headers.get("bearer") {
        Some(token) => token.to_str().unwrap(),
        None => return (StatusCode::UNAUTHORIZED, Json(None)),
    };
    debug!("Auth Token: {:?}", auth_token);

    let user = tank.new_user(&nickname).await.unwrap();

    (StatusCode::CREATED, Json(Some(user)))
}

pub async fn get_user(State(tank): State<Tank>, Path(user_id): Path<String>) -> impl IntoResponse {
    debug!("GET USER - User ID: {:?}", user_id);

    if let Ok(user_id) = user_id.parse::<Uuid>() {
        let user = tank.get_user(&user_id).await;

        return match user {
            Ok(user) => (StatusCode::OK, Json(Some(user))),
            Err(_) => (StatusCode::NOT_FOUND, Json(None)),
        };
    } else {
        return (StatusCode::BAD_REQUEST, Json(None));
    }
}

pub async fn get_users(State(tank): State<Tank>) -> impl IntoResponse {
    debug!("GET USERS");

    let users = tank.get_users().await.unwrap();
    Json(users)
}

pub async fn update_user(
    State(tank): State<Tank>,
    Path(user_id): Path<String>,
    Json(update_user): Json<UpdateUser>,
) -> impl IntoResponse {
    debug!(
        "PUT USER - User ID: {:?}, UpdateUser: {:?}",
        user_id, update_user
    );

    tank.update_user(&user_id.parse().unwrap(), &update_user)
        .await
        .unwrap()
}

pub async fn remove_user(
    State(tank): State<Tank>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    debug!("DELETE USER - User ID: {:?}", user_id);

    tank.remove_user(&user_id.parse().unwrap()).await.unwrap()
}
