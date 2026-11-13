use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;

pub mod model;
pub mod rankings;
pub mod read;

pub fn get_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(read::get_agents))
        .routes(routes!(read::get_ranks))
        .routes(routes!(read::get_maps))
        .routes(routes!(read::get_my_rankings))
        .routes(routes!(read::get_user_rankings))
        .routes(routes!(read::get_my_agent_ranking))
        .routes(routes!(read::get_user_agent_ranking))
}
