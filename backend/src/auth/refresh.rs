use axum::{Json, extract::State, response::IntoResponse};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::PgInterval, query_as};
use uuid::Uuid;

use crate::{
    AppState,
    auth::sessions::{Session, Token, TokenPair},
    error::Error,
};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct RefreshRequest {
    refresh_token: String,
    riot_name: String,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct RefreshResponse {
    pub id: Uuid,

    pub created_at: DateTime<Utc>,

    pub refreshed_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub access_expires_at: DateTime<Utc>,

    #[schema(
        example = "Qm7Kx2pN9vRt5YwH3cDf8LzA1sUe6GjB4oMi0XqTnPk",
        content_encoding = "^[A-Za-z0-9_-]{43}$"
    )]
    pub access_token: String,

    #[schema(
        example = "Zp4Vn8Hs2LqR6xJt0BcW5mYk9DaF3uGe7Ni1XoTrQwM",
        content_encoding = "^[A-Za-z0-9_-]{43}$"
    )]
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/refresh",
    tag = "Authentication",
    summary = "Refresh tokens",
    description = "Refreshes the tokens. Returns access and refresh tokens.",
    request_body = RefreshRequest,
    responses(
        (
            status = 200,
            description = "Refresh successful",
            body = RefreshResponse
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
pub(crate) async fn refresh(
    State(app_state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> Result<impl IntoResponse, Error> {
    let tokens = TokenPair::random();
    let session = query_as!(
        Session,
        "UPDATE sessions
        SET
            access_token_hash = $3,
            refresh_token_hash = $4,
            refreshed_at = now(),
            access_expires_at = now() + $5
        FROM users
        WHERE
            users.riot_name = $2
            AND sessions.user_id = users.id
            AND sessions.refresh_token_hash = $1
            AND expires_at > now()
            AND revoked_at IS NULL
        RETURNING
            sessions.id,
            sessions.created_at,
            sessions.refreshed_at,
            sessions.expires_at,
            sessions.access_expires_at,
            sessions.access_token_hash,
            sessions.refresh_token_hash",
        &Token::from_base_64(&request.refresh_token)
            .ok_or(Error::Unauthorized)?
            .hash(),
        &request.riot_name,
        &tokens.access.hash(),
        &tokens.refresh.hash(),
        PgInterval::try_from(Duration::hours(1)).map_err(|_| Error::Internal)?,
    )
    .fetch_optional(app_state.pool)
    .await?
    .ok_or(Error::Unauthorized)?;

    // Prepare response
    let response = RefreshResponse {
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
