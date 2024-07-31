use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use crate::models::users::Users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub fn get_user_by_email<'a>(connection: &mut PooledConnection<ConnectionManager<PgConnection>>, user_email: &'a str) -> Option<Users> {
    users.filter(email.eq(user_email)).first::<Users>(connection).optional().unwrap_or_else(|err| {
        println!("Error occurred: {:?}", err);
        None
    })
}