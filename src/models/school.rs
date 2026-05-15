use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct School {
    pub id: i64,
    pub name: String,
    pub code: String,
}
