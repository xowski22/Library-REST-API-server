use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Book is not available for borrowing")]
    BookNotAvailable,
    
    #[error("User already has an active loan for this book")]
    DuplicateLoan,
    
    #[error("Email already exists")]
    EmailConflict,
    
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    
    #[error("Pool error: {0}")]
    Pool(#[from] deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>),
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BookNotAvailable => (StatusCode::BAD_REQUEST, "Book is not available for borrowing".to_string()),
            AppError::DuplicateLoan => (StatusCode::BAD_REQUEST, "User already has an active loan for this book".to_string()),
            AppError::EmailConflict => (StatusCode::CONFLICT, "Email already in use".to_string()),
            AppError::Database(err) => {
                tracing::error!("Database error: {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            },
            AppError::Pool(err) => {
                tracing::error!("Pool error: {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Connection pool error".to_string())
            },
            AppError::Internal(err) => {
                tracing::error!("Internal server error: {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}