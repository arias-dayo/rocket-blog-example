use crate::schema::posts;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="posts"]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn create(post: Post, connection: &SqliteConnection) -> Post {
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(connection)
            .expect("記事の投稿に失敗しました。");

        posts::table.order(posts::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &SqliteConnection) -> Vec<Post> {
        posts::table.order(posts::id).load::<Post>(connection).unwrap()
    }

    pub fn update(id: i32, post: Post, connection: &SqliteConnection) -> bool {
        diesel::update(posts::table.find(id)).set(&post).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(posts::table.find(id)).execute(connection).is_ok()
    }
}