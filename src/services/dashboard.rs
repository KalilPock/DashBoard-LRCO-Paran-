use sqlx::SqlitePool;
use serde::Serialize;
use crate::models::school::School;

#[derive(Serialize)]
pub struct DashboardData {
    pub schools: Vec<School>,
    pub upcoming_assessments: Vec<AssessmentSummary>,
}

#[derive(Serialize)]
pub struct AssessmentSummary {
    pub school_name: String,
    pub subject: String,
    pub date: String,
}

pub struct DashboardService {
    db: SqlitePool,
}

impl DashboardService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn get_dashboard_data(&self) -> Result<DashboardData, Box<dyn std::error::Error>> {
        let schools = sqlx::query_as::<_, School>("SELECT id, name, lrco_id FROM schools")
            .fetch_all(&self.db)
            .await?;

        let upcoming_assessments = sqlx::query_as!(
            AssessmentSummary,
            r#"
            SELECT s.name as school_name, c.subject, a.date
            FROM assessments a
            JOIN classes c ON a.class_id = c.id
            JOIN schools s ON c.school_id = s.id
            ORDER BY a.date ASC
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(DashboardData {
            schools,
            upcoming_assessments,
        })
    }
}
