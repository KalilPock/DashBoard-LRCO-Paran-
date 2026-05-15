use sqlx::sqlite::SqlitePool;
use std::fs;

pub async fn init_db(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;
    
    // Apply schema
    let schema = fs::read_to_string("src/models/schema.sql")
        .expect("Failed to read schema.sql");
    
    sqlx::query(&schema).execute(&pool).await?;
    
    Ok(pool)
}
