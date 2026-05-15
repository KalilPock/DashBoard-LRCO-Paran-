use axum::{extract::State, response::IntoResponse};
use crate::AppState;

pub async fn sync_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let service = state.sync_service;
    match service.sync_all().await {
        Ok(_) => (axum::http::StatusCode::OK, "Sync completed successfully").into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error during sync").into_response(),
    }
}
