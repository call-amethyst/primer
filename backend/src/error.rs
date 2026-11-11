use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("database")]
    Database(#[from] sqlx::error::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("internal")]
    Internal,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
