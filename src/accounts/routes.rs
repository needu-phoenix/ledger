use super::handlers;
use crate::app::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_account))
        .route("/{id}/balance", get(handlers::get_balance))
}
