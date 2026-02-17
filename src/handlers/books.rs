use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    db::DbPool, errors::AppError, models::book::NewBook, services::book_service::BookService
};

pub async fn create_book(
    State(pool): State<DbPool>, 
    Json(new_book): Json<NewBook>
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let book = BookService::create(&pool, new_book).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(book))))
}

pub async fn list_books(
    State(pool): State<DbPool>
) -> Result<Json<serde_json::Value>, AppError> {
    let books = BookService::list_all(&pool).await?;
    Ok(Json(serde_json::json!(books)))
}

pub async fn get_book(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let book = BookService::get_by_id(&pool, id).await?;
    Ok(Json(serde_json::json!(book)))
}

pub async fn delete_book(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>
) -> Result<StatusCode, AppError> {
    BookService::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}