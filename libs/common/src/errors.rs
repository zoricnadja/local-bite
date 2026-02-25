use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg)      => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized(msg)  => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(msg)     => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::BadRequest(msg)    => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg)      => (StatusCode::CONFLICT, msg.clone()),
            AppError::Internal(_)        => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
            AppError::Database(e)        => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
            }
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::Internal(anyhow::anyhow!(e))
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        AppError::Internal(anyhow::anyhow!(e))
    }
}
pub type AppResult<T> = Result<T, AppError>;
