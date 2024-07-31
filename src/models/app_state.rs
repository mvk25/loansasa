use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use std::sync::Arc;

pub struct AppState {
    pub pool: Arc<Pool<ConnectionManager<PgConnection>>>
}