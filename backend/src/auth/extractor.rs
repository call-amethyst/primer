use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

use crate::{AppState, auth::sessions::Token, error::Error, user::User};

pub struct AuthenticatedUser(pub User);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(auth) = parts.headers.get(AUTHORIZATION) {
            let token = extract_bearer_token(auth.to_str().map_err(|_| Error::Unauthorized)?)
                .ok_or(Error::Unauthorized)?;
            let token = Token::from_base_64(token).ok_or(Error::Unauthorized)?;
            state.sessions.user(&token).await.map(AuthenticatedUser)
        } else {
            Err(Error::Unauthorized)
        }
    }
}

fn extract_bearer_token(header: &str) -> Option<&str> {
    let (scheme, token) = header.split_once(' ')?;

    if scheme.eq_ignore_ascii_case("bearer") && !token.is_empty() {
        Some(token)
    } else {
        None
    }
}
