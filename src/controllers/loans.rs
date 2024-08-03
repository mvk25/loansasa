use actix_session::Session;
use actix_web::{web::{self, get}, HttpResponse};
use askama::Template;
use chrono::NaiveDateTime;
use diesel::{RunQueryDsl, SelectableHelper};
use crate::{db_operations::users::get_user_by_email, models::{app_state::AppState, loans::*, ui::NewLoanTemplate}, schema::loans::loan};
use crate::schema::loans;
use diesel::result::Error as E;

pub async fn new_loan_page(session: Session, state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let result = match session.get::<String>("user_email") {
        Ok(Some(user_email)) => {
            let mut connection = state.pool.get().map_err(|_| {
                actix_web::error::ErrorInternalServerError("Couldn't get a connection from pool")
            }).unwrap();

            let user = get_user_by_email(&mut connection, &user_email);
            let template = NewLoanTemplate { error: None, user: user };
            Ok(HttpResponse::Ok().content_type("text/html").body(template.render().unwrap()))
        },
        Ok(None) => Ok(HttpResponse::Found().append_header((actix_web::http::header::LOCATION, "/login")).finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError("User not Found"))
    };
    result
}

pub async fn new_loan(form: web::Form<NewLoanForm>, session: Session, state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    // Get a session from the session user_email
    println!("Here we are. We got to the post handler start");
    let result = match session.get::<String>("user_email") {
        Ok(Some(user_email)) => {
            let mut connection = state.pool.get().map_err(|_| {
                actix_web::error::ErrorInternalServerError("Couldn't get a connection from pool")
            }).unwrap();
            // Get user from the user_email session, by getting a connection from the connection pool

            match get_user_by_email(&mut connection, &user_email) {
                Some(user) => {
                    println!("Here we are, at the apex of a dilemma");

                    let deadline = NaiveDateTime::parse_from_str(&form.deadline, "%Y-%m-%dT%H:%M").map_err(|err| {
                        println!("{:?}", err.kind());
                        actix_web::error::ErrorInternalServerError(err)
                    })?;
                    // Populate the NewLoan struct
                    let newloan = NewLoan {
                        loan: form.loan.clone(),
                        amount: form.amount.clone(),
                        upper_limit: user.loan_limit,
                        deadline,
                        users_id: user.id
                    };
                    // Insert row into the database using our connection
                    match diesel::insert_into(loans::table).values(newloan).returning(Loans::as_returning()).get_result(&mut connection) {
                        Ok(_) => {
                            Ok(HttpResponse::Found().append_header((actix_web::http::header::LOCATION, "/dashboard")).finish())
                        }
                        Err(E::DatabaseError(_, _)) => {
                            Err(actix_web::error::ErrorInternalServerError("Session error"))
                        }
                        Err(_) => {
                            Err(actix_web::error::ErrorInternalServerError("Database insertion error"))
                        }
                    }
                }
                None => Err(actix_web::error::ErrorInternalServerError("User not Found"))

            }

        },
        Ok(None) => Ok(HttpResponse::Found().append_header((actix_web::http::header::LOCATION, "/login")).finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError("User not Found"))
    };
    result
    // Create a html form which is populated by the web::Form extractor, make sure to add in
    // the dropdown for the laon type
    // On success, route to the /dashboard page
    // If you encounter an error, return the get page of the /new-loan, probably a server error, returning
    // the respective error.
}