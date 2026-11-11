pub mod auth;
pub mod db;
pub mod error;
pub mod openapi;
pub mod user;

use color_eyre::eyre::{Context, Result};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    auth::{
        login::{__path_login, login},
        sessions::SessionRepository,
    },
    openapi::ApiDoc,
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
    let auth = OpenApiRouter::new().routes(routes!(login));

    let api = OpenApiRouter::new()
        .nest("/auth", auth)
        .routes(routes!(hello));

    let app = OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/api", api);

    // Extract the Axum router and generated OpenAPI specification
    let (app, openapi) = app.split_for_parts();

    #[cfg(debug_assertions)]
    let app = app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi));

    let app = app.with_state(app_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .wrap_err("Couldn't bind to Port 3000!")?;
    axum::serve(listener, app)
        .await
        .wrap_err("Couldn't start app!")?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = "General",
    responses(
        (status = 200, description = "Hello response", body = String)
    )
)]
async fn hello() -> &'static str {
    "Hello, world!"
}
