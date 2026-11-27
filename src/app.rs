use crate::accounts;
use crate::health::health;
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
        .with_state(state)
}
