pub mod auth;
pub mod db;
pub mod error;
pub mod user;

use axum::{
    Router,
    routing::{get, post},
};
use color_eyre::eyre::{Context, Result};
use sqlx::PgPool;

use crate::{
    auth::{login::login, sessions::SessionRepository},
    user::UserRepository,
};

#[derive(Clone)]
pub struct AppState {
    users: UserRepository,
    sessions: SessionRepository,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let pool = db::connect().await?;
    let pool: &'static PgPool = Box::leak(Box::new(pool));
    let app_state = AppState {
        users: UserRepository::new(pool),
        sessions: SessionRepository::new(pool),
    };
    let auth = Router::new().route("/login", post(login));
    let api = Router::new()
        .nest("/auth", auth)
        .route("/hello", get(hello));

    let app = Router::new().nest("/api", api).with_state(app_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .wrap_err("Couldn't bind to Port 3000!")?;
    axum::serve(listener, app)
        .await
        .wrap_err("Couldn't start app!")?;
    Ok(())
}

// basic handler that responds with a static string
async fn hello() -> &'static str {
    "Hello World from the backend!"
}
