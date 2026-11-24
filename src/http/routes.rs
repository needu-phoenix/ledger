use super::{AppState, handlers};
use axum::{Router, routing::{get, post}};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/accounts", post(handlers::create_account))
        .with_state(state)
}
