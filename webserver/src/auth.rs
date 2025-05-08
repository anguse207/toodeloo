use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use toodeloo_core::token::Token;
use toodeloo_tank::pg::Tank;
use tracing::*;

pub async fn auth_middleware(
    State(tank): State<Tank>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    const ALLOWED_PATHS: [&str; 4] = [
        "/users/create",
        "/users/login",
        "/users/logout",
        "/users/test",
    ];

    // Check if the request path is in the allowed paths
    if ALLOWED_PATHS.iter().any(|&path| req.uri().path() == path) {
        println!("Auth Middleware Bypass for {}", req.uri().path());
        return next.run(req).await;
    }

    // Extract and validate the authorization token
    if let Some(auth_header) = req.headers().get("Bearer") {
        if let Ok(auth_token) = auth_header.to_str() {
            if let Ok(token) = tank
                .read_token(auth_token.parse().unwrap_or_default())
                .await
            {
                println!("User ID: {:?}", token);
                req.extensions_mut().insert(token);
                return next.run(req).await;
            }
        }
    }

    // Redirect to login if token is missing or invalid
    debug!("Invalid or missing token");
    Redirect::permanent("/login").into_response()
}

async fn _auth_test_handler(Extension(token): Extension<Token>) -> impl IntoResponse {
    format!("Token: {:?}", token)
}
