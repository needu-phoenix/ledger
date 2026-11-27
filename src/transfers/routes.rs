use super::handlers;
use crate::app::AppState;
use axum::{Router, routing::{post, get}};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_transfer))
        .route("/{id}/balance", get(handlers::get_balance))
}
