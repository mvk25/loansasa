pub mod controllers;
pub mod models;
pub mod schema;
pub mod db_operations;

use std::sync::Arc;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::{Key, SameSite}, web, App, HttpResponse, HttpServer, Responder};
use db_operations::db::establish_pool_conn;
use models::app_state::AppState;
use crate::controllers::users::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {pool: Arc::new(establish_pool_conn())}))
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .build()
            )
            .route("/register", web::get().to(register_page))
            .route("/register", web::post().to(register_user))
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login_user))
            .route("/dashboard", web::get().to(dashboard))
    }).bind("127.0.0.1:8080")?.run().await
}
