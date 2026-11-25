use axum::{Json, http::StatusCode};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateAccountRequest {
    pub name: String,
    pub currency: Option<String>,
}

pub async fn create_account(Json(req): Json<CreateAccountRequest>) -> StatusCode {
    info!(name = %req.name, "create account request received");
    StatusCode::CREATED
}
