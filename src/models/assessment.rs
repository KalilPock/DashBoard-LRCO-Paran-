use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Assessment {
    pub id: i64,
    pub class_id: i64,
    pub date: String,
    pub subject: String,
}
