use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::{Html, IntoResponse}, routing::get, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use toodeloo_tank::pg::Tank;
use tracing::info;

use crate::routes::RouterType;

const CLIENT_ID: &str = "1371559638784278568";
const CLIENT_SECRET: &str = "ent7proy53eWSso5f2Acz7QfW-0iL3hB";
const REDIRECT_URL: &str = "http://localhost:1337/auth/discord/callback";

pub fn routes() -> RouterType {
    Router::new()
        .route("/callback", get(callback))

}

/*
TODO: Use discord for authentication
# USERS table
user_id (from oauth)
oauth_provider
username
deleted_at
^ user_id is linked to the TOKENS table

# ^ future expansion
created_at
last_login
email
admin / permissions

# Flow
- User clicks on "Login with Discord" button
- User authenticates and returns to <callback_url>
- Use the access token to fetch user information from Discord
- Store the user information in the database / update if exists
- Generate a session token and return it to the user
- User can now use the session token to authenticate with the API
- User can log out by deleting the session token
*/

#[derive(Serialize, Deserialize, Debug)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

async fn callback(
    State(tank): State<Tank>,
    Query(CallbackQuery { code }): Query<CallbackQuery>,
) -> impl IntoResponse {
    let client = Arc::new(Client::new());

    info!("Received callback with code: {}", code);

    // Exchange the authorization code for an access token
    let access_token = client
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
            ("grant_type", "authorization_code"),
            ("redirect_uri", REDIRECT_URL),
            ("code", &code),
        ])
        .send()
        .await
        .unwrap()
        .json::<AccessToken>()
        .await
        .unwrap();

    info!("Received token response: {:?}", access_token);

    // Use the access token to fetch the user's information
    let user_info = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(&access_token.access_token)
        .send()
        .await
        .unwrap()
        .json::<UserInfo>()
        .await
        .unwrap();

    // Generate the avatar URL
    let avatar_url = match &user_info.avatar {
        Some(avatar) => format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            user_info.id, avatar
        ),
        None => "https://cdn.discordapp.com/embed/avatars/0.png".to_string(), // Default avatar
    };

    // Return an HTML response with the user's name and avatar
    Html(format!(
        r#"
        <html>
            <body>
                <h1>Welcome, {username}#{discriminator}!</h1>
                <img src="{avatar_url}" alt="Avatar" />
            </body>
        </html>
        "#,
        username = user_info.username,
        discriminator = user_info.discriminator,
        avatar_url = avatar_url
    ))
    // format!(
    //     "Hello, {}! Your Discord ID is {}.",
    //     user_info.username, user_info.id
    // )
}