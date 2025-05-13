use std::sync::Arc;

use axum::{
    extract::{Query, State}, response::{AppendHeaders, Html, IntoResponse, Redirect}, routing::get, Json, Router
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use toodeloo_core::user::OauthProvider;
use toodeloo_tank::pg::Tank;
use tracing::*;

use crate::routes::RouterType;

const CLIENT_ID: &str = "1371559638784278568";
const CLIENT_SECRET: &str = "ent7proy53eWSso5f2Acz7QfW-0iL3hB";
const REDIRECT_URL: &str = "http://localhost:1337/auth/discord/callback";

pub fn routes() -> RouterType {
    Router::new().route("/callback", get(callback))
}

/*
TODO: Use discord for authentication
# USERS table
user_id (from oauth)
oauth_provider
nickname
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

#[derive(Serialize, Deserialize, Debug)]
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

    debug!("Received user info: {:?}", user_info);

    // Login or create a new user in the database
    let token = match tank.read_user_by_oauth_id(&user_info.id).await {
        Ok(user) => {
            info!("Generating token for user: {:?}", &user_info.username);
            // Generate a session token for the user
            tank.create_token(user.id, core::time::Duration::from_secs(60 * 60 * 24 * 7))
                .await
                .unwrap()
        }
        Err(_) => {
            info!("User not found, creating new user");
            let user_id = tank
                .create_user(&user_info.id, OauthProvider::Discord, &user_info.username)
                .await
                .unwrap();

            info!("Generating token for user: {:?}", &user_info.username);
            tank.create_token(user_id, core::time::Duration::from_secs(60 * 60 * 24 * 7))
                .await
                .unwrap()
        }
    };

    // Add a header for setting the cookie
    let cookie_header = format!(
        "Bearer={}; HttpOnly; Path=/; Max-Age={}",
        token.id,
        i64::MAX // Maximum possible value for Max-Age
    );
    let headers = AppendHeaders([(axum::http::header::SET_COOKIE, cookie_header)]);    

    (headers, Redirect::to("/"))
}

fn get_avatar_url(user_info: &UserInfo) -> String {
    match &user_info.avatar {
        Some(avatar) => format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            user_info.id, avatar
        ),
        None => "https://cdn.discordapp.com/embed/avatars/0.png".to_string(), // Default avatar
    }
}
