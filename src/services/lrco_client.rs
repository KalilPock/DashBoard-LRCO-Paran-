use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SchoolDTO {
    pub id: String,
    pub name: String,
}

pub struct LrcoClient {
    client: reqwest::Client,
    api_key: String,
}

impl LrcoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn fetch_schools(&self) -> Result<Vec<SchoolDTO>, reqwest::Error> {
        // Mocking the call structure as actual endpoint details were not provided
        // In a real implementation, this would use self.api_key for auth
        Ok(vec![])
    }
}
