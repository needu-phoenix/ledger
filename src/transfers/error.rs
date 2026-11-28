use crate::error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("amount must be a positive whole number of pence")]
    InvalidAmount,

    #[error("from account same as to account")]
    SameAccount,

    #[error("account not found")]
    AccountNotFound,

    #[error("accounts' currencies differ")]
    CurrencyMismatch,

    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for TransferError {
    fn into_response(self) -> Response {
        match self {
            TransferError::InvalidAmount | TransferError::SameAccount => {
                error::json_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            TransferError::AccountNotFound => {
                error::json_error(StatusCode::NOT_FOUND, self.to_string())
            }
            TransferError::CurrencyMismatch => {
                error::json_error(StatusCode::CONFLICT, self.to_string())
            }
            TransferError::InsufficientFunds => {
                error::json_error(StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            TransferError::Database(e) => {
                error!(error = %e, "transfer database error");
                error::json_error(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        }
    }
}
