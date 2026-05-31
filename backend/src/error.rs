//! Unified error type that renders as a JSON HTTP response.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("unknown entity: {0}")]
    UnknownEntity(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("not implemented: {0}")]
    NotImplemented(&'static str),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AppError {
    /// Map each variant to its HTTP status.
    fn status(&self) -> StatusCode {
        todo!("map AppError variants to StatusCode")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Render as `{ "error": ... }`; hide internal details for 5xx.
        todo!("build JSON error response from self.status()")
    }
}
