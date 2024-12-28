use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use toodeloo_core::tank_traits::UserTank;
use tracing::*;
use uuid::Uuid;

use crate::tank_sqlite::Tank;

pub async fn new_user(
    State(tank): State<Tank>,
    Path(nickname): Path<String>,
) -> impl IntoResponse {
    debug!("GET USER - User ID: {:?}", nickname);
    let user = tank.new_user(&nickname).await.unwrap();

    Json(user)
}

pub async fn get_user(
    State(tank): State<Tank>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    debug!("GET USER - User ID: {:?}", user_id);
    let user = tank.get_user(&user_id.parse().unwrap()).await.unwrap();
    Json(user)
}

pub async fn get_users(State(tank): State<Tank>) -> impl IntoResponse {
    debug!("GET USERS");
    let users = tank.get_users().await.unwrap();
    Json(users)
}

#[derive(Deserialize)]
pub struct UpdateUser {
    nickname: String,
    token: Uuid,
}

pub async fn update_user(
    State(tank): State<Tank>,
    Path(user_id): Path<String>,
    Json(update_user): Json<UpdateUser>,
) -> impl IntoResponse {
    debug!("PUT USER - User ID: {:?}", user_id);
    // let user = tank.update_user().await.unwrap();
    // Json(user)
}

pub async fn remove_user(
    State(tank): State<Tank>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    debug!("DELETE USER - User ID: {:?}", user_id);
    tank.remove_user(&user_id.parse().unwrap()).await.unwrap()
}
