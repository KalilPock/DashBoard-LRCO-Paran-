use axum::{extract::State, response::IntoResponse};
use crate::AppState;
use crate::ui::dashboard::DashboardTemplate;
use askama::Template;

pub async fn get_dashboard_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let service = state.dashboard_service;
    match service.get_dashboard_data().await {
        Ok(data) => {
            let template = DashboardTemplate { data };
            template.render().map(|html| axum::response::Html(html)).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR).into_response()
        },
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching data").into_response(),
    }
}
