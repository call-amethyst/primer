use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;

pub mod create;
pub mod delete;
pub mod model;
pub mod rankings;
pub mod read;
pub mod update;

pub fn get_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(read::get_agents))
        .routes(routes!(read::get_ranks))
        .routes(routes!(read::get_maps))
        .routes(routes!(read::get_my_rankings))
        .routes(routes!(read::get_user_rankings))
        .routes(routes!(read::get_my_agent_ranking))
        .routes(routes!(read::get_user_agent_ranking))
        .routes(routes!(create::create_ranking))
        .routes(routes!(update::put_agent_ranking))
        .routes(routes!(update::patch_agent_ranking))
        .routes(routes!(update::replace_rankings))
        .routes(routes!(update::patch_rankings))
        .routes(routes!(delete::delete_agent_ranking))
        .routes(routes!(delete::delete_map_rankings))
}
