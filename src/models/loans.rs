use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use diesel::prelude::*;
use crate::models::users::Users;
use serde::{Serialize, Deserialize};

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::LoanType)]
pub enum LoanType {
    Personal,
    Auto,
    Student,
    Mortgage,
    Payday,
    Msme,
}

impl ToSql<crate::schema::sql_types::LoanType, Pg> for LoanType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            LoanType::Personal => out.write_all(b"personal")?,
            LoanType::Auto => out.write_all(b"auto")?,
            LoanType::Student => out.write_all(b"student")?,
            LoanType::Mortgage => out.write_all(b"mortgage")?,
            LoanType::Payday => out.write_all(b"payday")?,
            LoanType::Msme => out.write_all(b"msme")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::LoanType, Pg> for LoanType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"personal" => Ok(LoanType::Personal),
            b"auto" => Ok(LoanType::Auto),
            b"student" => Ok(LoanType::Student),
            b"mortgage" => Ok(LoanType::Mortgage),
            b"payday" => Ok(LoanType::Payday),
            b"msme" => Ok(LoanType::Msme),

            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::StatusType)]
pub enum StatusType {
    Pending,
    Active,
    Paid,
    Overdue,
}
impl ToSql<crate::schema::sql_types::StatusType, Pg> for crate::models::loans::StatusType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            crate::models::loans::StatusType::Pending => out.write_all(b"pending")?,
            crate::models::loans::StatusType::Active => out.write_all(b"active")?,
            crate::models::loans::StatusType::Paid => out.write_all(b"paid")?,
            crate::models::loans::StatusType::Overdue => out.write_all(b"overdue")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::StatusType, Pg> for crate::models::loans::StatusType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(crate::models::loans::StatusType::Pending),
            b"active" => Ok(crate::models::loans::StatusType::Active),
            b"paid" => Ok(crate::models::loans::StatusType::Paid),
            b"overdue" => Ok(crate::models::loans::StatusType::Overdue),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Queryable, Selectable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(Users))]
#[diesel(table_name = crate::schema::loans)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loans {
    pub id: i32,
    pub loan: LoanType,
    pub amount: i32,
    pub upper_limit: i32,
    pub status: StatusType,
    pub deadline: chrono::NaiveDateTime,
    pub users_id: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoan {
    pub loan: LoanType,
    pub amount: i32,
    pub upper_limit: i32,
    pub deadline: chrono::NaiveDateTime,
    pub users_id: i32
}


#[derive(Serialize, Deserialize)]
pub struct NewLoanForm {
    pub loan: LoanType,
    pub amount: i32,
    pub deadline: String,
}