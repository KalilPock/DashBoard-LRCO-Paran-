use axum::{extract::State, Json, response::IntoResponse};
use sqlx::SqlitePool;
use crate::services::dashboard::get_dashboard_data;
use serde::Serialize;

#[derive(Serialize)]
struct DashboardResponse {
    schools: Vec<crate::models::school::School>,
    classes: Vec<crate::models::class::Class>,
    assessments: Vec<crate::models::assessment::Assessment>,
}

pub async fn dashboard_handler(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match get_dashboard_data(&pool).await {
        Ok((schools, classes, assessments)) => {
            Json(DashboardResponse { schools, classes, assessments }).into_response()
        }
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching data").into_response(),
    }
}
