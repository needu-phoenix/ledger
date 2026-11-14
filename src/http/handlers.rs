use super::routes::AppState;
use axum::{extract::State, http::StatusCode};
use tracing::error;

pub async fn health(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(error = %e, "health check: database unreachable");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
