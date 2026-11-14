use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    AppState,
    auth::extractor::AuthenticatedUser,
    error::Error,
    ranking::model::{Agent, Map},
};

/// Remove one agent ranking.
#[utoipa::path(
    delete,
    path = "/users/me/{map}/rankings/{agent}",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
            description = "Name of the map"),
        ("agent" = Agent, Path,
            description = "Agent whose ranking is removed"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Ranking removed"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ranking not found"),
    )
)]
pub(crate) async fn delete_agent_ranking(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path((map, agent)): Path<(Map, Agent)>,
) -> Result<StatusCode, Error> {
    state
        .rankings
        .delete_user_ranking(&user.id, &map, &agent)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Remove all rankings belonging to the authenticated user on a certain map.
#[utoipa::path(
    delete,
    path = "/users/{map}/me/rankings",
    tag = "Rankings",
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "All rankings removed"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Rankings not found"),
    )
)]
pub(crate) async fn delete_map_rankings(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path(map): Path<Map>,
) -> Result<StatusCode, Error> {
    state.rankings.delete_user_rankings(&user.id, &map).await?;
    Ok(StatusCode::NO_CONTENT)
}
