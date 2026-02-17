use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    db::DbPool, handlers::{books, loans, users}
};

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/api/books", get(books::list_books))
        .route("/api/books", post(books::create_book))
        .route("/api/books/{id}", get(books::get_book))
        .route("/api/books/{id}", delete(books::delete_book))
        .route("/api/users", get(users::list_users))
        .route("/api/users", post(users::create_user))
        .route("/api/users/{id}", get(users::get_user))
        .route("/api/loans", get(loans::list_loans))
        .route("/api/loans", post(loans::borrow_book))
        .route("/api/loans/{loan_id}/return", put(loans::return_book))
        .route("/api/users/{user_id}/loans", get(loans::list_user_loans))
        .with_state(pool)
}