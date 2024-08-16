use askama::Template;
use super::{loans::Loans, users::Users};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub error: Option<&'a str>
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate<'a> {
    pub error: Option<&'a str>
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub email: Option<String>,
    pub user: Option<Users>,
    pub loans: Option<Vec<Loans>>
}

#[derive(Template)]
#[template(path = "newloan.html")]
pub struct NewLoanTemplate {
    pub error: Option<String>,
    pub user: Option<Users>
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub user: Option<Users>
}
