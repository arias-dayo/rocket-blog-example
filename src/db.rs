use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use dotenv::dotenv;

// An alias to the type for a pool of Diesel Mysql Connection
pub type CONNECTION_POOL = Pool<ConnectionManager<SqliteConnection>>;

// DBのコネクションプールを作成する。
pub fn connect() -> CONNECTION_POOL {
    dotenv().ok();

    let database_URL: String = env::var("DATABASE_URL")
                                .expect("環境変数\"DATABASE_URL\"が設定されていません。");

    let manager = ConnectionManager::<SqliteConnection>::new(database_URL);
    Pool::new(manager).expect("コネクションプールの作成に失敗しました。")
}

pub struct Connection(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<CONNECTION_POOL>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
