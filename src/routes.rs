use crate::models::*;
use rocket_contrib::templates::{Template};

#[derive(Serialize)]
struct TemplateContext {
    page_title: String,
}

#[derive(Serialize)]
struct PostContext {
    page_title: String,
}

// index.html.hbsを表示する
#[get("/")]
pub fn index() -> Template {
    // Todo: postsを渡して、Template側はeachで記事の一覧を表示する
    Template::render("base/index", &TemplateContext {
        page_title: "ブログのタイトルが入るよ".to_string(),
    })
}

// DBから受け取ったtitle, textを元にページを表示する
#[get("/posts/<page>")]
pub fn posts(page: u64) -> Template {
    // TODO: DBからpostを取得するようにする
    let posts = posts::Posts {
        title: "記事のタイトル".to_string(),
        text: "記事の内容".to_string()
    };
    Template::render("base/posts", &PostContext {
        page_title: posts.title,
    })
}

// 404エラー
#[catch(404)]
pub fn not_found() -> Template {
    Template::render("error/404", &TemplateContext {
        page_title: "404エラー".to_string(),
    })
}