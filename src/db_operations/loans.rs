use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use crate::schema::loans;
use crate::models::loans::Loans;

pub fn get_user_loans(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, user_id: i32) -> QueryResult<Vec<Loans>> {
    loans::table
        .filter(loans::users_id.eq(user_id))
        .load::<Loans>(conn)
}