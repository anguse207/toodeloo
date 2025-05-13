use axum::{
    extract::{Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Redirect}, Extension
};
use toodeloo_core::token::Token;
use toodeloo_tank::pg::Tank;
use tracing::*;

pub mod discord;

pub async fn auth_middleware(
    State(tank): State<Tank>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    // List of allowed paths that do not require authentication
    const ALLOWED_PATHS: [&str; 3] = [
        "/api/users/create",
        "/api/users/login",
        "/auth/discord/callback",
    ];

    // Check if the request path is in the allowed paths
    if ALLOWED_PATHS.iter().any(|&path| req.uri().path() == path) {
        debug!("Auth Middleware Bypass for {}", req.uri().path());
        return next.run(req).await;
    }

    // Extract and validate the authorization token
    if let Some(cookie_header) = req.headers().get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse the cookie header to extract the auth-token
            if let Some(auth_token) = cookie_str
                .split(';')
                .find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with("auth-token=") {
                        Some(cookie.trim_start_matches("auth-token=").to_string())
                    } else {
                        None
                    }
                })
            {
                if let Ok(token) = tank
                    .read_token(auth_token.parse().unwrap_or_default())
                    .await
                {
                    // Check if the token is valid
                    if token.is_valid() {
                        debug!("User ID: {:?}", token);
                        req.extensions_mut().insert(token);
                        return next.run(req).await;
                    }
                }
            }
        }
    }

    // Redirect to login if token is missing or invalid
    debug!("Invalid or missing token");
    (StatusCode::UNAUTHORIZED, Redirect::to("/login")).into_response()
}

async fn _auth_test_handler(Extension(token): Extension<Token>) -> impl IntoResponse {
    format!("Token: {:?}", token)
}