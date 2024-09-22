use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use std::io::Write;
use diesel::prelude::*;
use crate::models::users::Users;
use serde::{Serialize, Deserialize};

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::EmploymentType)]
pub enum EmploymentType {
    FullTime,
    PartTime,
    SelfEmployed,
}


impl ToSql<crate::schema::sql_types::EmploymentType, Pg> for EmploymentType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            EmploymentType::FullTime => out.write_all(b"full-time")?,
            EmploymentType::PartTime => out.write_all(b"part-time")?,
            EmploymentType::SelfEmployed => out.write_all(b"self-employed")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::EmploymentType, Pg> for EmploymentType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"full-tile" => Ok(EmploymentType::FullTime),
            b"part-time" => Ok(EmploymentType::PartTime),
            b"self-employed" => Ok(EmploymentType::SelfEmployed),

            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
#[derive(Queryable,Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Users))]
#[diesel(table_name = crate::schema::personal_details)]
pub struct PersonalDetails {
    id: i32,
    users_id: i32,
    phone_number: String,
    date_of_birth: chrono::NaiveDateTime,
    employment: EmploymentType,
    city: String,
    office_number: i32,
    monthly_salary: i32,
    company_name: String,
    office_email_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::personal_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewDetails {
    phone_number: i32,
    date_of_birth: chrono::NaiveDateTime,
    employment: EmploymentType,
    city: String,
    office_number: i32,
    monthly_salary: i32,
    company_name: String,
    office_email_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewFormDetails {
    phone_number: i32,
    date_of_birth: chrono::NaiveDateTime,
    employment: EmploymentType,
    city: String,
    office_number: i32,
    monthly_salary: i32,
    company_name: String,
    office_email_id: i32,
}