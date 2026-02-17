use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Selectable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub loan_date: NaiveDateTime,
    pub return_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::loans)]
pub struct NewLoan {
    pub user_id: Uuid,
    pub book_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct BorrowRequest {
    pub user_id: Uuid,
    pub book_id: Uuid,
}