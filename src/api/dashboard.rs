use sqlx::SqlitePool;
use crate::models::{school::School, class::Class, assessment::Assessment};

pub async fn get_dashboard_data(pool: &SqlitePool) -> Result<(Vec<School>, Vec<Class>, Vec<Assessment>), sqlx::Error> {
    let schools = sqlx::query_as!(School, "SELECT * FROM schools")
        .fetch_all(pool)
        .await?;
    
    let classes = sqlx::query_as!(Class, "SELECT * FROM classes")
        .fetch_all(pool)
        .await?;
        
    let assessments = sqlx::query_as!(Assessment, "SELECT * FROM assessments")
        .fetch_all(pool)
        .await?;
        
    Ok((schools, classes, assessments))
}
