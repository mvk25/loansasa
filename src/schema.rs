// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "loan_type"))]
    pub struct LoanType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoanType;

    loans (id) {
        id -> Int4,
        loan -> Nullable<LoanType>,
        upper_limit -> Int4,
        deadline -> Timestamp,
        user_id -> Nullable<Int4>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
        amount -> Int4,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        strikes -> Int4,
        loan_limit -> Int4,
        goodwill -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    users,
);
