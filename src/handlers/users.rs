use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    db::DbPool, errors::AppError, models::user::NewUser, services::user_service::UserService
};

pub async fn create_user(
    State(pool): State<DbPool>, 
    Json(new_user): Json<NewUser>
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let user = UserService::create(&pool, new_user).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(user))))
}

pub async fn list_users(
    State(pool): State<DbPool>
) -> Result<Json<serde_json::Value>, AppError> {
    let users = UserService::list_all(&pool).await?;
    Ok(Json(serde_json::json!(users)))
}

pub async fn get_user(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = UserService::get_by_id(&pool, id).await?;
    Ok(Json(serde_json::json!(user)))
}