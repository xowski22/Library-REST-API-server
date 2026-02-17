use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    db::DbPool, errors::AppError, models::loan::BorrowRequest, services::loan_service::LoanService
};

pub async fn borrow_book(
    State(pool): State<DbPool>, 
    Json(request): Json<BorrowRequest>
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let loan = LoanService::borrow(&pool, request.book_id, request.user_id).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(loan))))
}

pub async fn return_book(
    State(pool): State<DbPool>,
    Path(loan_id): Path<Uuid>
) -> Result<Json<serde_json::Value>, AppError> {
    let loan = LoanService::return_book(&pool, loan_id).await?;
    Ok(Json(serde_json::json!(loan)))
}

pub async fn list_loans(
    State(pool): State<DbPool>
) -> Result<Json<serde_json::Value>, AppError> {
    let loans = LoanService::list_all(&pool).await?;
    Ok(Json(serde_json::json!(loans)))
}

pub async fn list_user_loans(
    State(pool): State<DbPool>,
    Path(user_id): Path<Uuid>
) -> Result<Json<serde_json::Value>, AppError> {
    let loans = LoanService::list_by_user(&pool, user_id).await?;
    Ok(Json(serde_json::json!(loans)))
}