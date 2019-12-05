use crate::schema::post;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="post"]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub post_date: String
}

impl Post {
    pub fn create(post: Post, connection: &SqliteConnection) -> Post {
        diesel::insert_into(post::table)
            .values(&post)
            .execute(connection)
            .expect("記事の投稿に失敗しました。");

        post::table.order(post::id.desc()).first(connection).unwrap()
    }

    pub fn read_from_id(connection: &SqliteConnection, id: &i32) -> Post {
        post::table.filter(post::id.eq(&id)).order(post::id).first::<Post>(connection).unwrap()
    }

    pub fn update(id: i32, post: Post, connection: &SqliteConnection) -> bool {
        diesel::update(post::table.find(id)).set(&post).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(post::table.find(id)).execute(connection).is_ok()
    }
}