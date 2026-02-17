use diesel::prelude::*;
use uuid::Uuid;
use diesel_async::RunQueryDsl;

use crate::{
    db::pool::DbPool,
    errors::AppError, 
    models::user::{NewUser, User}, schema::users
};

pub struct UserService;

impl UserService {
    pub async fn create(pool: &DbPool, new_user: NewUser) -> Result<User, AppError> {
        let mut conn = pool.get().await?;
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation, 
                    _
                ) => AppError::EmailConflict,
                other => AppError::Database(other),
            })
    }

    pub async fn list_all(pool: &DbPool) -> Result<Vec<User>, AppError> {
        let mut conn = pool.get().await?;
        let result = users::table
            .select(User::as_select())
            .order(users::created_at.desc())
            .load(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<User, AppError> {
        let mut conn = pool.get().await?;
        users::table
            .find(id)
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|_| AppError::NotFound(format!("User {id} not found")))
    }
}