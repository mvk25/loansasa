use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use diesel::SelectableHelper;
use r2d2::LoggingErrorHandler;
use crate::db_operations::loans::get_user_loans;
use crate::db_operations::users::get_user_by_email;
use crate::models::personal_details::NewFormDetails;
use crate::models::{app_state::AppState, users::*};
use crate::models::ui::{DashboardTemplate, HomeTemplate, LoginTemplate, RegisterTemplate};
use crate::schema::users;
use askama::Template;
use diesel::prelude::*;
use diesel::result::Error as E;
use bcrypt::{hash, DEFAULT_COST, verify};

pub async fn home_page(session: Session, state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let result = match session.get::<String>("user_email") {
        Ok(Some(user_email)) => {
            let mut connection = state.pool.get().map_err(|_| {
                actix_web::error::ErrorInternalServerError("Database error")
            })?;

            match get_user_by_email(&mut connection, &user_email) {
                Some(user) => {
                    let template = HomeTemplate {
                        user: Some(user)
                    };                    
                    Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))
                }
                None => {
                    let template = HomeTemplate {
                        user: None
                    };                    
                    Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))
                }
            }
        }
        Ok(None) => {
            let template = HomeTemplate {
                user: None
            };                    
            Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))
        }
        Err(_) => {
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    };

    result
}

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate { error: Some(error) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn register_page() -> HttpResponse {
    let template = RegisterTemplate { error: None };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn register_user(form: web::Form<NewUserForm>, state: web::Data<AppState>) -> HttpResponse {
    if form.first_name.is_empty() || form.last_name.is_empty() || form.email.is_empty() || form.password.is_empty() {
        let template = RegisterTemplate { error: Some("All fields required") };
        return HttpResponse::BadRequest().body(template.render().unwrap());
    }

    let hashed_password = match hash(&form.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            println!("Error");
            return handle_register_error("Error hasiing password").await
        }
    };

    let new_user = NewUser {
        first_name: &form.first_name,
        last_name: &form.last_name,
        email: &form.email,
        password: &hashed_password,
    };
    
    let mut conn = state.pool.get().expect("Couldn't get db connection from pool");

    match diesel::insert_into(users::table).values(new_user).returning(Users::as_returning()).get_result(&mut conn) {
        Ok(_) => {
            HttpResponse::Found().append_header((actix_web::http::header::LOCATION, "/login")).finish()
        }
        Err(E::DatabaseError(_, _)) => {
            handle_register_error("Internal Server Error: Unable to insert the record").await
        }
        Err(_) => {
            handle_register_error("Internal Server Error: Unknown error").await
        }
    }

}


pub async fn personal_details(form: web::Form<NewFormDetails>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn login_page(error: Option<String>) -> impl Responder {
    let template = LoginTemplate { error: None};
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn login_user(form: web::Form<LoginForm>, state: web::Data<AppState>, session: Session) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = state.pool.get().expect("Unable to get a connection from pool");
    println!("Here we are at the apex of the dilemma");
    let user_exist = get_user_by_email(&mut conn, &form.email);
    match user_exist {
        Some(user) => {
            if verify(&form.password, &user.password).unwrap_or(false) {
                session.insert("user_email", &form.email);
                println!("Password match");
                Ok(HttpResponse::Found().append_header((actix_web::http::header::LOCATION, "/dashboard")).finish())
            } else {
                let error_msg = "Wrong password";
                let template = LoginTemplate { error: Some(error_msg) };
                Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))
            }
        }
        None => {
            let error_msg = "User not found";
            let template = LoginTemplate { error: Some(error_msg) };
            Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))
        }
    }
}

pub async fn dashboard(state: web::Data<AppState>, session: Session) -> Result<HttpResponse, actix_web::Error> {
    let result = match session.get::<String>("user_email") {
        Ok(Some(user_email)) => {
            let mut connection = state.pool.get().map_err(|_| {
                actix_web::error::ErrorInternalServerError("Database error")
            })?;

            match get_user_by_email(&mut connection, &user_email) {
                Some(user) => {
                    let loans = get_user_loans(&mut connection, user.id).unwrap();
                    let dashboard_template = DashboardTemplate {
                        email: Some(user.email.to_string()),
                        user: Some(user),
                        loans: loans
                    };
                    println!("User found");
                    Ok(HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().unwrap()))
                }
                None => {
                    println!("User not found");
                    Ok(HttpResponse::NotFound().append_header((actix_web::http::header::LOCATION, "/login")).finish())
                }
            }
        }
        Ok(None) => {
            println!("No user email in session");
            Ok(HttpResponse::Found().append_header((actix_web::http::header::LOCATION, "/login")).finish())
        }
        Err(_) => {
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    };

    result
}