use tracing::{error, info};
use super::AppState;
use serde::Deserialize;
use axum::{Json, extract::State, http::StatusCode};


#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateAccountRequest {
    pub name: String,
    pub currency: Option<String>
}

pub async fn health(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(error = %e, "health check: database unreachable");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

pub async fn create_account(Json(req): Json<CreateAccountRequest>) -> StatusCode {
    info!("Request: {:?}", req);
    StatusCode::CREATED
}
