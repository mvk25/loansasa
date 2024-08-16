// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "loan_type"))]
    pub struct LoanType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_type"))]
    pub struct StatusType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoanType;
    use super::sql_types::StatusType;

    loans (id) {
        id -> Int4,
        loan -> LoanType,
        amount -> Int4,
        upper_limit -> Int4,
        status -> StatusType,
        deadline -> Timestamp,
        users_id -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        strikes -> Int4,
        loan_limit -> Int4,
        goodwill -> Int4,
        loans_taken -> Int4,
        successful_returns -> Int4,
        default_times -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(loans -> users (users_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    users,
);
