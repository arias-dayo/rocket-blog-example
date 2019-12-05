use crate::models::*;
use rocket_contrib::templates::{Template};
use rocket::response::Redirect;


#[derive(Serialize)]
struct LoginContext {
    title: String,
}

#[derive(Responder)]
pub enum EitherResponse {
    Template(Template),
    Redirect(Redirect),
}

// 入力したIDとパスワードをチェックし、正しければ"/admin"へRedirect
#[get("/login")]
pub fn login() -> EitherResponse {
    // TODO: IDとパスワードでDBを検索し、存在すれば処理を続行
    
    EitherResponse::Redirect(Redirect::to(uri!(admin)))
}

/* 
* 1. "/login"からリダイレクトされた場合
* 2. 現在のセッションがログイン履歴に存在する場合
* の2ケースで認証をパスする
*/
#[get("/admin")]
pub fn admin() -> Template {
    // TODO: ログイン履歴にセッションが存在するかチェック
    
    Template::render("admin/login", &LoginContext {
        title: "ログイン".to_string(),
    })
}