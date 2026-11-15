use axum::{
    Json, extract::{Path, State}, http::StatusCode 
};

use crate::{
    AppState, auth::extractor::AuthenticatedUser, error::Error, ranking::{model::{Agent, Map, PatchRankingsRequest, Ranking, RankingsResponse, ReplaceRankingsRequest, UpdateRankingRequest}, rankings::RankingUpdateResult},
};

impl From<RankingUpdateResult> for StatusCode {
    fn from(value: RankingUpdateResult) -> Self {
        match value {
            RankingUpdateResult::Created => Self::CREATED,
            RankingUpdateResult::Updated => Self::OK,
        }
    }
}

/// Create or replace a ranking for a specific agent.
///
/// Returns 201 when created, 200 when updated.
#[utoipa::path(
    put,
    path = "/users/me/{map}/rankings/{agent}",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
            description = "Name of the map"),
        ("agent" = Agent, Path,
            description = "Agent to rank"),
    ),
    security(("bearer_auth" = [])),
    request_body = UpdateRankingRequest,
    responses(
        (status = 200, description = "Ranking updated",
            body = Ranking),
        (status = 201, description = "Ranking created",
            body = Ranking),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    )
)]

pub(crate) async fn put_agent_ranking(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path((map, agent)): Path<(Map, Agent)>,
    Json(payload): Json<UpdateRankingRequest>,
) -> Result<(StatusCode, Json<Ranking>), Error> {
    let ranking = Ranking { agent, rank: payload.rank };
    let result = state.rankings.upsert_user_ranking(&user.id, &map, &ranking).await?;

    Ok((result.into(), Json(ranking)))
}

/// Update the rank of an already-ranked agent.
///
/// Returns 404 if the ranking does not exist.
#[utoipa::path(
    patch,
    path = "/users/me/{map}/rankings/{agent}",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
            description = "Name of the map"),
        ("agent" = Agent, Path,
            description = "Agent to update"),
    ),
    security(("bearer_auth" = [])),
    request_body = UpdateRankingRequest,
    responses(
        (status = 200, description = "Ranking updated",
            body = Ranking),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ranking not found"),
    )
)]
pub(crate) async fn patch_agent_ranking(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path((map, agent)): Path<(Map, Agent)>,
    Json(payload): Json<UpdateRankingRequest>,
) -> Result<Json<Ranking>, Error> {
    let ranking = Ranking { agent, rank: payload.rank };
    state.rankings.update_user_ranking(&user.id, &map, &ranking).await?;

    Ok(Json(ranking))
}

/// Replace the authenticated user's entire ranking list on a certain map.
///
/// Agents omitted from the request become unranked.
/// An empty list removes all rankings.
/// The replacement must be atomic.
#[utoipa::path(
    put,
    path = "/users/me/{map}/rankings",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
            description = "Name of the map"),
    ),
    security(("bearer_auth" = [])),
    request_body = ReplaceRankingsRequest,
    responses(
        (status = 200, description = "Rankings replaced",
            body = RankingsResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Duplicate agents"),
    
    )
)]
pub(crate) async fn replace_rankings(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path(map): Path<Map>,
    Json(payload): Json<ReplaceRankingsRequest>,
) -> Result<Json<RankingsResponse>, Error> {
    // Reject duplicate agents before modifying anything.
    let mut seen = std::collections::HashSet::new();

    for ranking in &payload.rankings {
        if !seen.insert(&ranking.agent) {
            return Err(Error::Duplicate);
        }
    }

    let mut tx = state.pool.begin().await?;

    // Remove all existing rankings for this map.
    sqlx::query!(
        r#"
        DELETE FROM rankings
        WHERE user_id = $1 AND map = $2
        "#,
        user.id,
        map ,
    )
    .execute(&mut *tx)
    .await?;

    // Insert the replacement rankings.
    for ranking in &payload.rankings {
        sqlx::query!(
            r#"
            INSERT INTO rankings (user_id, map, agent, rank)
            VALUES ($1, $2, $3, $4)
            "#,
            user.id,
            map,
            &ranking.agent,
            &ranking.rank,
        )
        .execute(&mut *tx)
        .await?;
    }

    // Read the resulting rankings before committing.
    let rankings = sqlx::query_as!(
        Ranking,
        r#"
        SELECT agent, rank
        FROM rankings
        WHERE user_id = $1 AND map = $2
        "#,
        user.id,
        map as _,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(RankingsResponse {
        user_id: user.id,
        rankings,
    }))
}

/// Partially modify multiple rankings on a certain map.
///
/// - upsert: Create or update rankings.
/// - remove: Delete rankings.
/// - Unmentioned agents remain unchanged.
/// - The operation must be atomic.
#[utoipa::path(
    patch,
    path = "/users/me/{map}/rankings",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
            description = "Name of the map"),
    ),
    security(("bearer_auth" = [])),
    request_body = PatchRankingsRequest,
    responses(
        (status = 200, description = "Rankings updated",
            body = RankingsResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Conflicting agents"),
    )
)]
pub(crate) async fn patch_rankings(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path(map): Path<Map>,
    Json(payload): Json<PatchRankingsRequest>,
) -> Result<Json<RankingsResponse>, Error> {
    // Reject duplicate or conflicting agents.
    let mut seen = std::collections::HashSet::new();

    for ranking in &payload.upsert {
        if !seen.insert(&ranking.agent) {
            return Err(Error::Duplicate);
        }
    }

    for agent in &payload.remove {
        if !seen.insert(agent) {
            return Err(Error::Duplicate);
        }
    }

    let mut tx = state.pool.begin().await?;

    // Remove specified rankings.
    for agent in &payload.remove {
        sqlx::query!(
            r#"
            DELETE FROM rankings
            WHERE user_id = $1
              AND map = $2
              AND agent = $3
            "#,
            user.id,
            map as _,
            agent as _,
        )
        .execute(&mut *tx)
        .await?;
    }

    // Insert or update specified rankings.
    for ranking in &payload.upsert {
        sqlx::query!(
            r#"
            INSERT INTO rankings (user_id, map, agent, rank)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, map, agent)
            DO UPDATE SET rank = EXCLUDED.rank
            "#,
            user.id,
            map as _,
            &ranking.agent as _,
            &ranking.rank as _,
        )
        .execute(&mut *tx)
        .await?;
    }

    // Fetch the updated ranking list.
    let rankings = sqlx::query_as!(
        Ranking,
        r#"
        SELECT agent, rank
        FROM rankings
        WHERE user_id = $1 AND map = $2
        "#,
        user.id,
        map as _,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(RankingsResponse {
        user_id: user.id,
        rankings,
    }))
}
