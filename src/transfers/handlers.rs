use super::{
    db,
    error::TransferError,
    model::{Balance, NewTransfer, Transfer},
};
use crate::app::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransferRequest {
    from_account_id: i64,
    to_account_id: i64,
    amount: i64,
}

pub async fn create_transfer(
    State(state): State<AppState>,
    Json(req): Json<TransferRequest>,
) -> Result<(StatusCode, Json<Transfer>), TransferError> {
    let t = NewTransfer::new(req.from_account_id, req.to_account_id, req.amount)?;

    let transfer = db::transfer(&state.pool, &t).await?;

    Ok((StatusCode::CREATED, Json(transfer)))
}

pub async fn get_balance(
    State(s): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Balance>), TransferError> {
    let balance = db::balance(&s.pool, id).await?;

    Ok((StatusCode::OK, Json(balance)))
}
