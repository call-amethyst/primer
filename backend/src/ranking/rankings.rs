use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::{
    error::Error,
    ranking::model::{Agent, Map, Ranking},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RankingUpdateResult {
    Created,
    Updated,
}

#[derive(Debug, Clone)]
pub(crate) struct RankingRepository {
    pool: &'static PgPool,
}

impl RankingRepository {
    #[must_use]
    pub(crate) const fn new(pool: &'static PgPool) -> Self {
        Self { pool }
    }

    /// Creates or updates an agent ranking depending on the mode.
    pub(crate) async fn create_user_ranking(
        &self,
        user_id: &Uuid,
        map: &Map,
        ranking: &Ranking,
    ) -> Result<(), Error> {
        query!(
            r#"
                INSERT INTO rankings
                    (user_id, map, agent, rank)
                VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            map,
            &ranking.agent,
            &ranking.rank,
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub(crate) async fn update_user_ranking(
        &self,
        user_id: &Uuid,
        map: &Map,
        ranking: &Ranking,
    ) -> Result<(), Error> {
        let result = query!(
            r#"
                UPDATE rankings
                SET rank = $4
                WHERE user_id = $1
                AND map = $2
                AND agent = $3
            "#,
            user_id,
            map,
            &ranking.agent,
            &ranking.rank,
        )
        .execute(self.pool)
        .await?;

        if result.rows_affected() == 0 {
            Err(Error::NotFound)
        } else {
            Ok(())
        }
    }

    pub(crate) async fn upsert_user_ranking(
        &self,
        user_id: &Uuid,
        map: &Map,
        ranking: &Ranking,
    ) -> Result<RankingUpdateResult, Error> {
        let result = query!(
            r#"
                INSERT INTO rankings
                    (user_id, map, agent, rank)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (user_id, map, agent)
                DO UPDATE SET
                    rank = EXCLUDED.rank
                RETURNING (old.id IS NULL) AS "created!"
                "#,
            user_id,
            map,
            &ranking.agent,
            &ranking.rank,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(if result.created {
            RankingUpdateResult::Created
        } else {
            RankingUpdateResult::Updated
        })
    }

    /// Fetches all rankings for a user on a map.
    pub(crate) async fn user_rankings(
        &self,
        user_id: &Uuid,
        map: &Map,
    ) -> Result<Vec<Ranking>, Error> {
        query_as!(
            Ranking,
            r#"
            SELECT agent, rank
            FROM rankings
            WHERE user_id = $1 AND map = $2
            "#,
            user_id,
            map,
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Database)
    }

    /// Fetches one agent ranking.
    pub(crate) async fn user_ranking(
        &self,
        user_id: &Uuid,
        map: &Map,
        agent: &Agent,
    ) -> Result<Option<Ranking>, Error> {
        query_as!(
            Ranking,
            r#"
            SELECT agent, rank
            FROM rankings
            WHERE user_id = $1
              AND map = $2
              AND agent = $3
            "#,
            user_id,
            map,
            agent,
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Database)
    }

    /// Deletes one agent ranking.
    ///
    /// Returns true if a row was deleted.
    pub(crate) async fn delete_user_ranking(
        &self,
        user_id: &Uuid,
        map: &Map,
        agent: &Agent,
    ) -> Result<(), Error> {
        let result = query!(
            r#"
            DELETE FROM rankings
            WHERE user_id = $1
              AND map = $2
              AND agent = $3
            "#,
            user_id,
            map,
            agent,
        )
        .execute(self.pool)
        .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    /// Deletes every ranking belonging to a user on a map.
    ///
    /// Returns the number of deleted rankings.
    pub(crate) async fn delete_user_rankings(
        &self,
        user_id: &Uuid,
        map: &Map,
    ) -> Result<u64, Error> {
        let result = query!(
            r#"
            DELETE FROM rankings
            WHERE user_id = $1
              AND map = $2
            "#,
            user_id,
            map,
        )
        .execute(self.pool)
        .await?;

        if result.rows_affected() > 0 {
            Ok(result.rows_affected())
        } else {
            Err(Error::NotFound)
        }
    }
}
