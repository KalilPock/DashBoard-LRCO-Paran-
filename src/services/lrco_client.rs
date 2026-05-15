use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LrcoConfig {
    pub api_url: String,
    pub api_key: String,
}

pub struct LrcoClient {
    client: Client,
    config: LrcoConfig,
}

impl LrcoClient {
    pub fn new(config: LrcoConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn fetch_data(&self, endpoint: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/{}", self.config.api_url, endpoint);
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;
        
        response.json().await
    }
}
