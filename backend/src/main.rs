use std::convert::Infallible;
use reqwest::Client;
use warp::body::form;
use warp::Filter;
use crate::handlers::get_account_data;

mod etherscan;
mod handlers;
mod client;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = Client::new(); // This should be the initialized reqwest client
    let client_filter = warp::any().map(move || client.clone());

    let address_info = warp::path!("address")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16)) // limiting the content length
        .and(warp::body::json())
        .and(client_filter.clone())
        .and_then(get_account_data);


// Combine all the routes
    let routes = address_info;
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type"]);

    warp::serve(routes.with(cors)).run(([127, 0, 0, 1], 8080)).await;
}
