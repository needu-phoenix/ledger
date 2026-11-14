use std::time::Duration;

use crate::{config::Config, http::handlers};
use axum::{Router, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn new(config: &Config) -> anyhow::Result<AppState> {
        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.database_url)
            .await?;
        Ok(AppState { pool })
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .with_state(state)
}
