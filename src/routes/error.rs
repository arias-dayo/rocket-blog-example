use crate::models::*;
use rocket_contrib::templates::{Template};
use rocket::Request;

#[derive(Serialize)]
struct ErrorContext {
    title: String,
}

// 404エラー
#[catch(404)]
pub fn not_found() -> Template {
    Template::render("error/404", &ErrorContext {
        title: "404エラー".to_string(),
    })
}

#[catch(500)]
pub fn server_error(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}