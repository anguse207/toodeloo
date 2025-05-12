use axum::{
    extract::{Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Redirect}, Extension
};
use toodeloo_core::token::Token;
use toodeloo_tank::pg::Tank;
use tracing::*;

pub async fn auth_middleware(
    State(tank): State<Tank>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    // List of allowed paths that do not require authentication
    const ALLOWED_PATHS: [&str; 3] = [
        "/api/users/create",
        "/api/users/login",
        "/auth/discord/callback",
    ];

    // Check if the request path is in the allowed paths
    if ALLOWED_PATHS.iter().any(|&path| req.uri().path() == path) {
        debug!("Auth Middleware Bypass for {}", req.uri().path());
        return Ok(next.run(req).await);
    }

    // Extract and validate the authorization token
    if let Some(auth_header) = req.headers().get("Bearer") {
        if let Ok(auth_token) = auth_header.to_str() {
            if let Ok(token) = tank
                .read_token(auth_token.parse().unwrap_or_default())
                .await
            {
                // Check if the token is valid
                if token.is_valid() {
                    debug!("User ID: {:?}", token);
                    req.extensions_mut().insert(token);
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    // Redirect to login if token is missing or invalid
    debug!("Invalid or missing token");
    return Err((StatusCode::UNAUTHORIZED, "You are not allowed to read this list"));
}

async fn _auth_test_handler(Extension(token): Extension<Token>) -> impl IntoResponse {
    format!("Token: {:?}", token)
}
