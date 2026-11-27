use super::model::MAX_NAME_LEN;
use crate::error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("account name must not be empty")]
    EmptyName,

    #[error("account name must be at most {MAX_NAME_LEN} characters")]
    NameTooLong,

    #[error("currency must be 3 letters, got {0:?}")]
    InvalidCurrency(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        match self {
            AccountError::EmptyName
            | AccountError::InvalidCurrency(_)
            | AccountError::NameTooLong => {
                error::json_error(StatusCode::BAD_REQUEST, self.to_string())
            }
            AccountError::Database(e) => {
                error!(error = %e, "account database error");
                error::json_error(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        }
    }
}
