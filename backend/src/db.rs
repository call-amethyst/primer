use std::env;

use color_eyre::eyre::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub(crate) async fn connect() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?)
}
