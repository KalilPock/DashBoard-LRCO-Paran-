use sqlx::SqlitePool;
use crate::models::school::School;
use crate::models::class::Class;
use crate::models::assessment::Assessment;

pub async fn get_dashboard_data(pool: &SqlitePool) -> Result<(Vec<School>, Vec<Class>, Vec<Assessment>), sqlx::Error> {
    let schools = sqlx::query_as::<_, School>("SELECT * FROM schools").fetch_all(pool).await?;
    let classes = sqlx::query_as::<_, Class>("SELECT * FROM classes").fetch_all(pool).await?;
    let assessments = sqlx::query_as::<_, Assessment>("SELECT * FROM assessments").fetch_all(pool).await?;
    
    Ok((schools, classes, assessments))
}
