use reqwest::Client;
use serde::Serialize;

const SERVER_URL: &str = "http://localhost:8080/address";

#[derive(Serialize)]
pub struct SearchForm {
    pub address: String,
    pub start_block: String,
    pub end_block: String,
}

pub async fn get_address_info(address: &str, start_block: &str, end_block: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let body = SearchForm { address: address.to_string(), start_block: start_block.to_string(), end_block: end_block.to_string() };
    let resp = client
        .post(SERVER_URL)
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}
