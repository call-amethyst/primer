use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, postgres::types::PgInterval, query_as};
use uuid::Uuid;

use crate::{error::Error, user::User};

#[non_exhaustive]
pub struct TokenPair {
    pub access: Token,
    pub refresh: Token,
}

impl TokenPair {
    #[must_use]
    pub fn random() -> Self {
        Self {
            access: Token::random(),
            refresh: Token::random(),
        }
    }
}

pub struct Token(pub [u8; 32]);

impl Token {
    #[must_use]
    pub fn random() -> Self {
        Self(rand::random())
    }

    #[must_use]
    pub fn from_base_64(value: &str) -> Option<Self> {
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(value)
            .ok()
            .and_then(|value| value.try_into().ok())
            .map(Token)
    }

    #[must_use]
    pub fn hash(&self) -> [u8; 32] {
        Sha256::digest(self.0).into()
    }

    #[must_use]
    pub fn base_64(&self) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.0)
    }
}

#[non_exhaustive]
pub struct Session {
    pub id: Uuid,

    pub created_at: DateTime<Utc>,
    pub refreshed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub access_expires_at: DateTime<Utc>,

    pub access_token_hash: Vec<u8>,
    pub refresh_token_hash: Vec<u8>,
}

#[derive(Clone)]
pub(crate) struct SessionRepository {
    pool: &'static PgPool,
}

impl SessionRepository {
    #[must_use]
    pub const fn new(pool: &'static PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user: &User,
        session_validity: Duration,
        refresh_validity: Duration,
        tokens: &TokenPair,
    ) -> Result<Session, Error> {
        let session = query_as!(
            Session,
            "INSERT INTO sessions 
                        (user_id, expires_at, access_expires_at, 
                        access_token_hash, refresh_token_hash) 
            VALUES ($1, now() + $2, now() + $3, $4, $5)
            RETURNING id, created_at, refreshed_at, 
                        expires_at, access_expires_at, 
                        access_token_hash, refresh_token_hash",
            user.id,
            PgInterval::try_from(session_validity).ok(),
            PgInterval::try_from(refresh_validity).ok(),
            &tokens.access.hash(),
            &tokens.refresh.hash(),
        )
        .fetch_one(self.pool)
        .await?;
        Ok(session)
    }

    pub async fn user(&self, token: &Token) -> Result<User, Error> {
        query_as!(
            User,
            "SELECT users.id, users.name, users.riot_name, users.password_hash 
            FROM users 
            JOIN sessions 
            ON sessions.user_id=users.id
            WHERE sessions.access_token_hash=$1
            AND expires_at > now()
            AND revoked_at IS NULL",
            &token.hash()
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(Error::NotFound)
    }

    pub async fn sessions(&self, user: &User) -> Result<Vec<Session>, Error> {
        let sessions = query_as!(
            Session,
            "SELECT id, created_at, refreshed_at, 
                    expires_at, access_expires_at, 
                    access_token_hash, refresh_token_hash
            FROM sessions
            WHERE user_id=$1
            ",
            user.id
        )
        .fetch_all(self.pool)
        .await?;
        Ok(sessions)
    }
}
