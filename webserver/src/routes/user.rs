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
    Router::new()
        // .route("/", get(fetch_all))
        .route(
        "/",
        post(create)
            .get(fetch)
            .put(update)
            .delete(remove_user),
    )
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

    debug!("POST USER - Nick: {:?}", name);

    let user = tank.new_user(&name, &pass).await.unwrap();

    (StatusCode::CREATED, Json(Some(user)))
}

async fn login() {
    debug!("POST USER - Login");

    // tank.login_user(&user_id.parse().unwrap()).await.unwrap()
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
