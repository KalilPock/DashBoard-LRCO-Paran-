use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Class {
    pub id: String,
    pub school_id: String,
    pub subject: String,
    pub schedule: String,
    pub lrco_id: String,
}
