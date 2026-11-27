use crate::{
    accounts::{
        db,
        error::AccountError,
        model::{Account, AccountName, Currency},
    },
    app::AppState,
};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateAccountRequest {
    pub name: String,
    pub currency: Option<String>,
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Account>), AccountError> {
    let name = AccountName::parse(&req.name)?;
    let currency = match req.currency.as_deref() {
        Some(c) => Currency::parse(c)?,
        None => Currency::default(),
    };

    let account = db::create(&state.pool, &name, &currency).await?;

    Ok((StatusCode::CREATED, Json(account)))
}
