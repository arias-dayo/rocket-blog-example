use crate::models::*;
use rocket_contrib::templates::{Template};

#[derive(Serialize)]
struct ErrorContext {
    page_title: String,
}

// 404エラー
#[catch(404)]
pub fn not_found() -> Template {
    Template::render("error/404", &ErrorContext {
        page_title: "404エラー".to_string(),
    })
}