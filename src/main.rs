mod models {
    pub mod school;
    pub mod class;
    pub mod assessment;
}
mod services {
    pub mod db;
    pub mod dashboard;
    pub mod lrco_client;
    pub mod sync;
}
mod api {
    pub mod dashboard;
}
mod ui {
    pub mod dashboard;
}

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use crate::services::db::establish_connection;
use crate::api::dashboard::dashboard_handler;

#[tokio::main]
async fn main() {
    // 1. Inicializa a conexão com o SQLite
    let pool = establish_connection().await;

    // 2. Configura as rotas da aplicação
    let app = Router::new()
        // Rota da API para pegar os dados do dashboard
        .route("/api/dashboard", get(dashboard_handler))
        // Rota principal
        .route("/", get(|| async { "DashBoard LRCO - Servidor Rodando!" }))
        .with_state(pool);

    // 3. Define o endereço e inicia o servidor (Sintaxe do Axum 0.7)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Servidor rodando em http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}