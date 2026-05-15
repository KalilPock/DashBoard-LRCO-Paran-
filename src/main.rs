use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use crate::services::{dashboard::DashboardService, sync::SyncService, lrco_client::{LrcoClient, LrcoConfig}};

#[derive(Clone)]
pub struct AppState {
    pub dashboard_service: Arc<DashboardService>,
    pub sync_service: Arc<SyncService>,
}

mod models {
    pub mod school;
    pub mod class;
    pub mod assessment;
}
mod services {
    pub mod db;
    pub mod lrco_client;
    pub mod sync;
    pub mod dashboard;
}
mod api {
    pub mod dashboard;
    pub mod sync;
}
mod ui {
    pub mod dashboard;
}

#[tokio::main]
async fn main() {
    let pool = services::db::init_db("my_project.db").await.expect("Failed to init DB");
    
    let lrco_config = LrcoConfig { api_url: "http://api.lrco.pr.gov.br".into(), api_key: "secret".into() };
    let lrco_client = LrcoClient::new(lrco_config);

    let state = AppState {
        dashboard_service: Arc::new(DashboardService::new(pool.clone())),
        sync_service: Arc::new(SyncService::new(lrco_client, pool.clone())),
    };

    let app = Router::new()
        .route("/", get(|| async { "Dashboard Server Running" }))
        .route("/dashboard", get(api::dashboard::get_dashboard_handler))
        .route("/sync", post(api::sync::sync_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running at http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();
}
