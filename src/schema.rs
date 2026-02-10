// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Uuid,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 150]
        author -> Varchar,
        publication_year -> Int4,
        available -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    loans (id) {
        id -> Uuid,
        book_id -> Uuid,
        user_id -> Uuid,
        loan_date -> Timestamp,
        return_date -> Nullable<Date>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        #[max_length = 150]
        surname -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(loans -> books (book_id));
diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(books, loans, users,);
