use sqlx::{Executor, Pool, Postgres};
use tokio::sync::OnceCell;

static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

async fn init_db() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    pool.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGSERIAL PRIMARY KEY,
            username TEXT NOT NULL,
            display_name TEXT NOT NULL
        )",
    )
    .await
    .expect("Failed to run migrations");

    pool
}

/// Get or initialize the database connection pool.
pub async fn get_db() -> &'static Pool<Postgres> {
    DB.get_or_init(init_db).await
}
