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
    Router::new().route("/", get(fetch_all)).route(
        "/{nickname}",
        post(create)
            .get(fetch)
            .put(update)
            .delete(remove_user),
    )
}

pub async fn create(
    State(tank): State<Tank>,
    Path(name): Path<String>,
    Path(pass): Path<String>,
) -> impl IntoResponse {
    debug!("POST USER - Nick: {:?}", name);

    let user = tank.new_user(&name, &pass).await.unwrap();

    (StatusCode::CREATED, Json(Some(user)))
}

pub async fn fetch(State(tank): State<Tank>, Path(user_id): Path<String>) -> impl IntoResponse {
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

pub async fn fetch_all(State(tank): State<Tank>) -> impl IntoResponse {
    debug!("GET USERS");

    let users = tank.get_users().await.unwrap();
    Json(users)
}

pub async fn update(
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
