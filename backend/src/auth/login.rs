use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, response::IntoResponse};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, auth::sessions::TokenPair, error::Error};

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    riot_name: String,
    password: String,
}

#[derive(Serialize)]
pub(crate) struct LoginResponse {
    pub id: Uuid,

    pub created_at: DateTime<Utc>,
    pub refreshed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub access_expires_at: DateTime<Utc>,

    pub access_token: String,
    pub refresh_token: String,
}

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
