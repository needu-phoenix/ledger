use crate::health::health;
use crate::{accounts, transfers};
use axum::{Router, routing::get};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/accounts", accounts::routes::routes())
        .nest("/transfers", transfers::routes::routes())
        .with_state(state)
}
