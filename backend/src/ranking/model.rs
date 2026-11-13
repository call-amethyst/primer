use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use strum::EnumIter;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, EnumIter, Type,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "game_map", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Map {
    Bind,
    Haven,
    Split,
    Ascent,
    Icebox,
    Breeze,
    Fracture,
    Pearl,
    Lotus,
    Sunset,
    Abyss,
    Corrode,
    Summit,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, EnumIter, Type,
)]
#[sqlx(type_name = "agent")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Agent {
    Astra,
    Breach,
    Brimstone,
    Chamber,
    Clove,
    Cypher,
    Deadlock,
    Fade,
    Gekko,
    Harbor,
    Hush,
    Iso,
    Jett,
    KAYO,
    Killjoy,
    Miks,
    Neon,
    Omen,
    Phoenix,
    Raze,
    Reyna,
    Sage,
    Skye,
    Sova,
    Tejo,
    Veto,
    Viper,
    Vyse,
    Waylay,
    Yoru,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, EnumIter, Type,
)]
#[sqlx(type_name = "agent_rank")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Rank {
    Main,
    Good,
    Average,
    Bad,
    NotPlayed,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Ranking {
    pub agent: Agent,
    pub rank: Rank,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRankingRequest {
    pub agent: Agent,
    pub rank: Rank,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRankingRequest {
    pub rank: Rank,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RankingsResponse {
    pub user_id: Uuid,
    pub rankings: Vec<Ranking>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReplaceRankingsRequest {
    pub rankings: Vec<CreateRankingRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchRankingsRequest {
    #[serde(default)]
    pub upsert: Vec<CreateRankingRequest>,

    #[serde(default)]
    pub remove: Vec<Agent>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AgentsResponse {
    pub agents: Vec<Agent>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RanksResponse {
    pub ranks: Vec<Rank>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MapsResponse {
    pub maps: Vec<Map>,
}
