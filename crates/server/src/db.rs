use sqlx::{Executor, Pool, Sqlite};
use tokio::sync::OnceCell;

static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

async fn init_db() -> Pool<Sqlite> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://app.db?mode=rwc".to_string());

    let pool = sqlx::sqlite::SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    pool.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            display_name TEXT NOT NULL
        )",
    )
    .await
    .expect("Failed to run migrations");

    pool
}

/// Get or initialize the database connection pool.
pub async fn get_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(init_db).await
}
