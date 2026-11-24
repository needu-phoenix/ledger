use crate::config::Config;
use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn connect(config: &Config) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .context("Could not connect to Postgres: is it running and is DATABASE_URL correct")?;

    Ok(pool)
}
