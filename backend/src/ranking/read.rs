use axum::{
    Json,
    extract::{Path, State},
};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    AppState,
    auth::extractor::AuthenticatedUser,
    error::Error,
    ranking::model::{
        Agent, AgentsResponse, Map, MapsResponse, Rank, Ranking, RankingsResponse, RanksResponse,
    },
};

/// List all available agents.
#[utoipa::path(
    get,
    path = "/api/v1/agents",
    tag = "Agents",
    responses(
        (status = 200, description = "Available agents",
            body = AgentsResponse),
    )
)]
pub(crate) async fn get_agents() -> Result<Json<AgentsResponse>, Error> {
    let agents = Agent::iter().collect();
    let response = AgentsResponse { agents };
    Ok(Json(response))
}

/// List all available ranks.
#[utoipa::path(
    get,
    path = "/api/v1/ranks",
    tag = "Rankings",
    responses(
        (status = 200, description = "Available ranks",
            body = RanksResponse),
    )
)]
pub(crate) async fn get_ranks() -> Result<Json<RanksResponse>, Error> {
    let ranks = Rank::iter().collect();
    let response = RanksResponse { ranks };
    Ok(Json(response))
}

/// List all available maps.
#[utoipa::path(
    get,
    path = "/api/v1/maps",
    tag = "Maps",
    responses(
        (status = 200, description = "Available maps",
            body = MapsResponse),
    )
)]
pub(crate) async fn get_maps() -> Result<Json<MapsResponse>, Error> {
    let maps = Map::iter().collect();
    let response = MapsResponse { maps };
    Ok(Json(response))
}

/// Get the authenticated user's rankings.
#[utoipa::path(
    get,
    path = "/api/v1/users/me/{map}/rankings",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
             description = "Name of the map"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User rankings",
            body = RankingsResponse),
        (status = 401, description = "Unauthorized"),
    )
)]
pub(crate) async fn get_my_rankings(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path(map): Path<Map>,
) -> Result<Json<RankingsResponse>, Error> {
    state
        .rankings
        .user_rankings(&user.id, &map)
        .await
        .map(|rankings| {
            Json(RankingsResponse {
                user_id: user.id,
                rankings,
            })
        })
}

/// Get another user's rankings.
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}/{map}/rankings",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
             description = "Name of the map"),
        ("user_id" = Uuid, Path,
            description = "ID of the ranking owner"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User rankings",
            body = RankingsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    )
)]
pub(crate) async fn get_user_rankings(
    AuthenticatedUser(_user): AuthenticatedUser,
    State(state): State<AppState>,
    Path((user_id, map)): Path<(Uuid, Map)>,
) -> Result<Json<RankingsResponse>, Error> {
    state
        .rankings
        .user_rankings(&user_id, &map)
        .await
        .map(|rankings| Json(RankingsResponse { user_id, rankings }))
}

/// Get the authenticated user's ranking for one agent.
#[utoipa::path(
    get,
    path = "/api/v1/users/me/{map}/rankings/{agent}",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
             description = "Name of the map"),
        ("agent" = Agent, Path,
            description = "Agent enum value"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Agent ranking",
            body = Ranking),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ranking not found"),
    )
)]
pub(crate) async fn get_my_agent_ranking(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path((map, agent)): Path<(Map, Agent)>,
) -> Result<Json<Ranking>, Error> {
    state
        .rankings
        .user_ranking(&user.id, &map, &agent)
        .await
        .and_then(|ranking| ranking.ok_or(Error::NotFound).map(Json))
}

/// Get another user's ranking for one agent.
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}/{map}/rankings/{agent}",
    tag = "Rankings",
    params(
        ("user_id" = Uuid, Path,
            description = "ID of the ranking owner"),
        ("map" = Map, Path,
             description = "Name of the map"),
        ("agent" = Agent, Path,
            description = "Agent enum value"),
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Agent ranking",
            body = Ranking),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ranking not found"),
    )
)]
pub(crate) async fn get_user_agent_ranking(
    AuthenticatedUser(_user): AuthenticatedUser,
    State(state): State<AppState>,
    Path((user_id, map, agent)): Path<(Uuid, Map, Agent)>,
) -> Result<Json<Ranking>, Error> {
    state
        .rankings
        .user_ranking(&user_id, &map, &agent)
        .await
        .and_then(|ranking| ranking.ok_or(Error::NotFound).map(Json))
}
