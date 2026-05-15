use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Assessment {
    pub id: String,
    pub class_id: String,
    pub date: String,
    pub r#type: String,
    pub lrco_id: String,
}
