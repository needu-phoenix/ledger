use super::{AppState, handlers};
use axum::{Router, routing::get};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .with_state(state)
}
