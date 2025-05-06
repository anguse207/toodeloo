use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
// use serde::Deserialize;
use toodeloo_core::user::UpdateUser;
use toodeloo_tank::sqlite::Tank;
use tracing::*;
use uuid::Uuid;

pub fn routes() -> Router<Tank> {
    Router::new().route("/", get(get_users)).route(
        "/{nickname}",
        post(new_user)
            .get(get_user)
            .put(update_user)
            .delete(remove_user),
    )
}

pub async fn new_user(
    State(tank): State<Tank>,
    Path(nickname): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    debug!("POST USER - Nick: {:?}", nickname);

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

        match user {
            Ok(user) => (StatusCode::OK, Json(Some(user))),
            Err(_) => (StatusCode::NOT_FOUND, Json(None)),
        }
    } else {
        (StatusCode::BAD_REQUEST, Json(None))
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
