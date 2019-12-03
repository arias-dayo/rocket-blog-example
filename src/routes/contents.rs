use crate::models::*;
use rocket_contrib::templates::{Template};

#[derive(Serialize)]
struct IndexContext {
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
    Template::render("base/index", &IndexContext {
        page_title: "ブログのタイトルが入るよ".to_string(),
    })
}

// DBから受け取ったtitle, textを元にページを表示する
#[get("/posts/<id>")]
pub fn posts(id: u64) -> Template {
    // TODO: DBからpostを取得するようにする
    
    Template::render("base/posts", &PostContext {
        page_title: "記事のタイトルが入るよ".to_string(),
    })
}
