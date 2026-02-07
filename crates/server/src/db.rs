use sqlx::{Pool, Postgres};
use tokio::sync::OnceCell;

static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

async fn init_db() -> Pool<Postgres> {
    // Load .env file if present (ignored in production where env vars are set directly).
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

/// Get or initialize the database connection pool.
pub async fn get_db() -> &'static Pool<Postgres> {
    DB.get_or_init(init_db).await
}
