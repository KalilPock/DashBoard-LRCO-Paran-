use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assessment {
    pub id: String,
    pub class_id: String,
    pub date: String,
    pub r#type: String,
    pub lrco_id: String,
}
