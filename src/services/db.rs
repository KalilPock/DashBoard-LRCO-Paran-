use sqlx::sqlite::SqlitePool;
use std::env;

pub async fn establish_connection() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:my_project.db".to_string());
    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool.")
}
