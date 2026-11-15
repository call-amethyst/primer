use argon2::{Argon2, PasswordHasher};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{AppState, error::Error};

#[derive(Deserialize, ToSchema)]
pub(crate) struct RegisterRequest {
    name: String,
    riot_name: String,

    #[schema(format = Password)]
    password: String,
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "Authentication",
    summary = "Register",
    description = "Registers a user with their respective data. Inserts the data and returns success.",
    request_body = RegisterRequest,
    responses(
        (
            status = 204,
            description = "Login successful",
        ),
        (
            status = 400,
            description = "Invalid request body"
        ),
        (
            status = 409,
            description = "Duplicate uuid or riot name"
        ),
        (
            status = 500,
            description = "Internal server error"
        )
    )
)]
pub(crate) async fn register(
    State(app_state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, Error> {
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(request.password.as_bytes())
        .map_err(|_| Error::BadRequest)?
        .to_string();

    app_state
        .users
        .insert(&request.name, &request.riot_name, &password_hash)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
