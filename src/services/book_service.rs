use diesel::prelude::*;
use uuid::Uuid;
use diesel_async::RunQueryDsl;

use crate::{
    db::DbPool,
    errors::AppError,
    models::book::{Book, NewBook},
    schema::books,
};

pub struct BookService;

impl BookService {
    pub async fn create(pool: &DbPool, new_book: NewBook) -> Result<Book, AppError> {
        let mut conn = pool.get().await?;
        let book = diesel::insert_into(books::table)
            .values(&new_book)
            .returning(Book::as_returning())
            .get_result(&mut conn)
            .await?;
        Ok(book)
    }

    pub async fn list_all(pool: &DbPool) -> Result<Vec<Book>, AppError> {
        let mut conn = pool.get().await?;
        let result = books::table
            .select(Book::as_select())
            .order(books::created_at.desc())
            .load::<Book>(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<Book, AppError> {
        let mut conn = pool.get().await?;
        let book = books::table
            .filter(books::id.eq(id))
            .select(Book::as_select())
            .first::<Book>(&mut conn)
            .await?;
        Ok(book)
    }

    pub async fn set_available(pool: &DbPool, id: Uuid, available: bool) -> Result<(), AppError> {
        let mut conn = pool.get().await?;
        diesel::update(books::table.find(id))
            .set(books::available.eq(available))
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> Result<(), AppError> {
        let mut conn = pool.get().await?;
        let deleted = diesel::delete(books::table.find(id)).execute(&mut conn).await?;
        if deleted == 0 {
            return Err(AppError::NotFound(format!("Book {id} not found")));
        }
        Ok(())
    }
}