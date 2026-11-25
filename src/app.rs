use crate::{accounts::handlers, health};
use sqlx::PgPool;

use axum::{
    Router,
    routing::{get, post},
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/accounts", post(handlers::create_account))
        .with_state(state)
}
