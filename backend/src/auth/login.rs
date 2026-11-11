use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, response::IntoResponse};
use chrono::Duration;
use serde::Deserialize;

use crate::{AppState, auth::sessions::TokenPair, error::Error};

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    riot_name: String,
    password: String,
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
    Ok(Json(session))
}
