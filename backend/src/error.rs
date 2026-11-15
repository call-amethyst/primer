use axum::{http::StatusCode, response::IntoResponse};
use sqlx::error::DatabaseError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("database")]
    Database(sqlx::error::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("duplicate")]
    Duplicate,
    #[error("bad request")]
    BadRequest,
    #[error("internal")]
    Internal,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            Self::Duplicate => StatusCode::CONFLICT.into_response(),
        }
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        if value
            .as_database_error()
            .is_some_and(DatabaseError::is_unique_violation)
        {
            Self::Duplicate
        } else {
            Self::Database(value)
        }
    }
}
