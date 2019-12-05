use rocket_contrib::templates::{Template};
use crate::db;

use crate::models::post::Post;

#[derive(Serialize)]
struct IndexContext {
    title: String,
}

// index.html.hbsを表示する
#[get("/")]
pub fn index() -> Template {
    // Todo: postsを渡して、Template側はeachで記事の一覧を表示する
    Template::render("base/index", &IndexContext {
        title: "ブログのタイトルが入るよ".to_string(),
    })
}

// DBから受け取ったtitle, textを元にページを表示する
#[get("/posts/<id>")]
pub fn posts(id: i32, connection: db::DbCon) -> Template {
    // TODO: DBからpostを取得するようにする
    let post = Post::read_from_id(&connection, &id);
    
    Template::render("base/posts", post)
}
