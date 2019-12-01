#[database("sqlite_database")]
pub struct LogsDbConn(diesel::SqliteConnection);