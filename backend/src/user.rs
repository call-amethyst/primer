use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::error::Error;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub riot_name: String,
    pub password_hash: String,
}

#[derive(Clone)]
pub(crate) struct UserRepository {
    pool: &'static PgPool,
}

impl UserRepository {
    #[must_use]
    pub const fn new(pool: &'static PgPool) -> Self {
        Self { pool }
    }

    pub async fn with_id(&self, id: &Uuid) -> Result<User, Error> {
        let user = query_as!(
            User,
            "SELECT id, name, riot_name, password_hash FROM users WHERE id=$1",
            id
        )
        .fetch_optional(self.pool)
        .await?;
        user.ok_or(Error::NotFound)
    }
    pub async fn with_riot_name(&self, riot_name: &str) -> Result<User, Error> {
        let user = query_as!(
            User,
            "SELECT id, name, riot_name, password_hash FROM users WHERE riot_name=$1",
            riot_name
        )
        .fetch_optional(self.pool)
        .await?;
        user.ok_or(Error::NotFound)
    }
}
