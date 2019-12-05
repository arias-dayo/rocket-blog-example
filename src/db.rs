use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

// DBのコネクションプールを作成する。
pub fn connect() -> ConnectionPool {
    let database_url: String = "file:./db.sqlite".to_string();

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("コネクションプールの作成に失敗しました。")
}

pub struct DbCon(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbCon {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<ConnectionPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbCon(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbCon {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
