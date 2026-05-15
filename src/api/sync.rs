use axum::{extract::State, Json, response::IntoResponse};
use sqlx::SqlitePool;
use crate::services::sync::synchronize_data;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SyncRequest {
    pub api_key: String,
}

pub async fn sync_handler(State(pool): State<SqlitePool>, Json(payload): Json<SyncRequest>) -> impl IntoResponse {
    match synchronize_data(&pool, &payload.api_key).await {
        Ok(_) => (axum::http::StatusCode::OK, "Sync successful").into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Sync failed").into_response(),
    }
}
