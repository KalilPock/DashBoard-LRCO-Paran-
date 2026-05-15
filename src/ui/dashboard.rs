use askama::Template;
use crate::services::dashboard::DashboardData;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub data: DashboardData,
}
