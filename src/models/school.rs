use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct School {
    pub id: String,
    pub name: String,
    pub lrco_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Class {
    pub id: String,
    pub school_id: String,
    pub subject: String,
    pub schedule: String,
    pub lrco_id: String,
}
