use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::{
    error::Error,
    ranking::model::{Agent, Map, Ranking},
};

#[derive(Debug, Clone)]
pub(crate) struct RankingRepository {
    pool: &'static PgPool,
}

impl RankingRepository {
    #[must_use]
    pub const fn new(pool: &'static PgPool) -> Self {
        Self { pool }
    }

    pub async fn user_rankings(&self, user_id: &Uuid, map: &Map) -> Result<Vec<Ranking>, Error> {
        query_as!(
            Ranking,
            r#"SELECT 
                agent, 
                rank
                FROM rankings 
            WHERE user_id=$1 AND map=$2
            "#,
            user_id,
            map
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Database)
    }

    pub async fn user_ranking(
        &self,
        user_id: &Uuid,
        map: &Map,
        agent: &Agent,
    ) -> Result<Option<Ranking>, Error> {
        query_as!(
            Ranking,
            r#"SELECT 
                agent, 
                rank 
                FROM rankings 
            WHERE user_id=$1 AND map=$2 AND agent=$3
            "#,
            user_id,
            map,
            agent,
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Database)
    }
}
