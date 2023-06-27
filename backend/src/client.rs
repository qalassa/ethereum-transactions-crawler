use reqwest::{header, Client};

pub fn get_client() -> Client {
    let headers = header::HeaderMap::new();
    let client = Client::builder()
        .default_headers(headers)
        .user_agent("EthereumApp")
        .build()
        .expect("Failed to build the client");

    client
}
