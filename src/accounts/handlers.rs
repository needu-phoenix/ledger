use crate::{
    accounts::{db, model::Account},
    app::AppState,
};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateAccountRequest {
    pub name: String,
    pub currency: Option<String>,
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Account>), StatusCode> {
    //info!(name = %req.name, "create account request received");
    let currency = &req.currency.unwrap_or("GBP".to_string());
    let account = db::create(&state.pool, &req.name, currency)
        .await
        .map_err(|e| {
            error!(error = % e, currency = %currency, "failed to create account");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(account)))
}
