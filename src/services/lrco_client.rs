use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LrcoData {
    // Expected LRCO response structure
}

pub async fn fetch_lrco_data(api_key: &str) -> Result<LrcoData, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://api.lrco.seed.pr.gov.br/data")
        .header("Authorization", api_key)
        .send()
        .await?
        .json::<LrcoData>()
        .await
}
