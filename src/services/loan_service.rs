use uuid::Uuid;
use diesel:: prelude::*;
use diesel_async::RunQueryDsl;
use chrono::Utc;

use crate::{
    db::DbPool,
    errors::AppError,
    models::loan::{Loan, NewLoan},
    schema::loans,
    services::book_service::BookService,
};

pub struct LoanService;

impl LoanService {
    pub async fn borrow(pool: &DbPool, book_id: Uuid, user_id: Uuid) -> Result<Loan, AppError> {
        let book = BookService::get_by_id(pool, book_id).await?;
        if !book.available {
            return Err(AppError::BookNotAvailable);
        }

        let mut conn = pool.get().await?;
        let active: i64 = loans::table
            .filter(loans::book_id.eq(book_id))
            .filter(loans::user_id.eq(user_id))
            .filter(loans::return_date.is_null())
            .count()
            .get_result(&mut conn)
            .await?;

        if active > 0 {
            return Err(AppError::DuplicateLoan);
        }

        let new_loan = NewLoan { user_id, book_id };
        let loan = diesel::insert_into(loans::table)
            .values(&new_loan)
            .returning(Loan::as_returning())
            .get_result(&mut conn)
            .await?;

        BookService::set_available(pool, book_id, false).await?;
        Ok(loan)

        
    }

    pub async fn return_book(pool: &DbPool, loan_id: Uuid) -> Result<Loan, AppError> {
        let mut conn = pool.get().await?;

        let loan: Loan = loans::table
            .find(loan_id)
            .select(Loan::as_select())
            .first(&mut conn)
            .await
            .map_err(|_| AppError::NotFound(format!("Loan {loan_id} not found")))?;

        if loan.return_date.is_some() {
            return Err(AppError::NotFound("Book already returned".to_string()));
        }
        
        let today = Utc::now().date_naive();
        let updated_loan = diesel::update(loans::table.find(loan_id))
            .set(loans::return_date.eq(today))
            .returning(Loan::as_returning())
            .get_result(&mut conn)
            .await?;

        BookService::set_available(pool, updated_loan.book_id, true).await?;
        Ok(updated_loan)
    }

    pub async fn list_all(pool: &DbPool) -> Result<Vec<Loan>, AppError> {
        let mut conn = pool.get().await?;
        let result = loans::table
            .select(Loan::as_select())
            .order(loans::loan_date.desc())
            .load(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn list_by_user(pool: &DbPool, user_id: Uuid) -> Result<Vec<Loan>, AppError> {
        let mut conn = pool.get().await?;
        let result = loans::table
            .filter(loans::user_id.eq(user_id))
            .select(Loan::as_select())
            .order(loans::created_at.desc())
            .load(&mut conn)
            .await?;
        Ok(result)
    }
}