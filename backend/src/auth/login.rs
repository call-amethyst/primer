use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, response::IntoResponse};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, auth::sessions::TokenPair, error::Error};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct LoginRequest {
    /// Riot name of the user
    riot_name: String,

    /// Account password
    #[schema(format = Password)]
    password: String,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct LoginResponse {
    /// Unique session identifier
    pub id: Uuid,

    /// Time the session was created
    pub created_at: DateTime<Utc>,

    /// Time the refresh token was last rotated
    pub refreshed_at: DateTime<Utc>,

    /// Time the session expires
    pub expires_at: DateTime<Utc>,

    /// Time the access token expires
    pub access_expires_at: DateTime<Utc>,

    /// Base64URL-encoded access token
    #[schema(
        example = "Qm7Kx2pN9vRt5YwH3cDf8LzA1sUe6GjB4oMi0XqTnPk",
        content_encoding = "^[A-Za-z0-9_-]{43}$"
    )]
    pub access_token: String,

    /// Base64URL-encoded refresh token
    #[schema(
        example = "Zp4Vn8Hs2LqR6xJt0BcW5mYk9DaF3uGe7Ni1XoTrQwM",
        content_encoding = "^[A-Za-z0-9_-]{43}$"
    )]
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "Authentication",
    summary = "Log in",
    description = "Authenticates a user with their riot name and password. Creates a session and returns access and refresh tokens.",
    request_body = LoginRequest,
    responses(
        (
            status = 200,
            description = "Login successful",
            body = LoginResponse
        ),
        (
            status = 400,
            description = "Invalid request body"
        ),
        (
            status = 401,
            description = "Invalid credentials"
        ),
        (
            status = 500,
            description = "Internal server error"
        )
    )
)]
pub(crate) async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, Error> {
    let user = app_state
        .users
        .with_riot_name(&request.riot_name)
        .await
        .map_err(|err| {
            if matches!(err, Error::NotFound) {
                Error::Unauthorized
            } else {
                err
            }
        })?;
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| Error::Internal)?;
    Argon2::default()
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Unauthorized)?;

    // Password verified
    let tokens = TokenPair::random();
    let session = app_state
        .sessions
        .create(Duration::days(30), Duration::hours(1), &tokens)
        .await?;

    // Prepare response
    let response = LoginResponse {
        id: session.id,

        created_at: session.created_at,
        refreshed_at: session.refreshed_at,
        expires_at: session.expires_at,
        access_expires_at: session.access_expires_at,

        access_token: tokens.access.base_64(),
        refresh_token: tokens.refresh.base_64(),
    };
    Ok(Json(response))
}
