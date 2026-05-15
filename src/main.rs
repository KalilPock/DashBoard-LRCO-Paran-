mod models {
    pub mod school;
    pub mod class;
    pub mod assessment;
}
mod services {
    pub mod db;
    pub mod lrco_client;
    pub mod sync;
}
mod api {
    pub mod dashboard;
}
mod ui {
    pub mod dashboard;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize database
    let pool = services::db::init_db("my_project.db").await?;

    // 2. Synchronize data (new)
    if let Ok(api_key) = std::env::var("LRCO_API_KEY") {
        services::sync::synchronize_data(&pool, &api_key).await?;
    }

    // 3. Fetch data
    let (schools, classes, assessments) = api::dashboard::get_dashboard_data(&pool).await?;

    // 4. Render dashboard
    ui::dashboard::render_dashboard(&schools, &classes, &assessments);

    Ok(())
}
