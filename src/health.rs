use crate::app::AppState;
use axum::{extract::State, http::StatusCode};

pub async fn health(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!(error = %e, "health check: database unreachable");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
