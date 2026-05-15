use crate::services::lrco_client::{LrcoClient};
use sqlx::SqlitePool;

pub struct SyncService {
    client: LrcoClient,
    db: SqlitePool,
}

impl SyncService {
    pub fn new(client: LrcoClient, db: SqlitePool) -> Self {
        Self { client, db }
    }

    pub async fn sync_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Exemplo: busca escolas
        let data = self.client.fetch_data("schools").await?;
        
        if let Some(schools) = data.as_array() {
            for school in schools {
                sqlx::query!(
                    "INSERT INTO schools (id, name, lrco_id) VALUES (?, ?, ?) 
                     ON CONFLICT(lrco_id) DO UPDATE SET name = excluded.name",
                    school["id"].as_str().unwrap_or(""),
                    school["name"].as_str().unwrap_or(""),
                    school["lrco_id"].as_str().unwrap_or("")
                )
                .execute(&self.db)
                .await?;
            }
        }
        
        Ok(())
    }
}
