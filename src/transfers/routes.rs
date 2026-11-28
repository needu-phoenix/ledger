use super::handlers;
use crate::app::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(handlers::create_transfer))
}
