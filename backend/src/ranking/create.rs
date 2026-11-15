use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    AppState,
    auth::extractor::AuthenticatedUser,
    error::Error,
    ranking::model::{CreateRankingRequest, Map, Ranking},
};

/// Create a ranking for an unranked agent.
///
/// Fails with 409 if the user already ranked the agent.
#[utoipa::path(
    post,
    path = "/users/me/{map}/rankings",
    tag = "Rankings",
    params(
        ("map" = Map, Path,
             description = "Name of the map"),
    ),
    security(("bearer_auth" = [])),
    request_body = CreateRankingRequest,
    responses(
        (status = 201, description = "Ranking created",
            body = Ranking),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Ranking already exists"),
    )
)]
pub(crate) async fn create_ranking(
    AuthenticatedUser(user): AuthenticatedUser,
    State(state): State<AppState>,
    Path(map): Path<Map>,
    Json(payload): Json<CreateRankingRequest>,
) -> Result<Json<Ranking>, Error> {
    let ranking = Ranking {
        agent: payload.agent,
        rank: payload.rank,
    };
    state
        .rankings
        .create_user_ranking(&user.id, &map, &ranking)
        .await?;
    Ok(Json(ranking))
}
