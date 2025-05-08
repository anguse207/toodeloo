use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use toodeloo_tank::pg::Tank;
use uuid::Uuid;

const DEBUG_AUTH: bool = true;

pub async fn auth_middleware(
    State(tank): State<Tank>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    if DEBUG_AUTH {
        println!("Auth Middleware Bypass for {}", req.uri().path());
        return next.run(req).await;
    }

    // Extract the authorization header
    let auth_token = match req.headers().get("Bearer") {
        Some(token) => token.to_str().unwrap(),
        None => return Redirect::temporary("/login").into_response(),
    };

    // look up the user ID from the token
    match tank.read_user_id_from_token(auth_token.parse().unwrap_or_default()).await {
        Ok(user_id) => {
            println!("User ID: {:?}", user_id);
            req.extensions_mut().insert(user_id);
        }
        Err(_) => {
            println!("Invalid token");
            return Redirect::permanent("/login").into_response();
        }
    }

    next.run(req).await
}

async fn _auth_test_handler(Extension(user_id): Extension<Uuid>) -> impl IntoResponse {
    format!("User ID: {}", user_id)
}
