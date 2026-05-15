use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Class {
    pub id: i64,
    pub school_id: i64,
    pub name: String,
}
