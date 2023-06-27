use crate::etherscan::fetch_account_data;
use warp::{http::StatusCode, Rejection, Reply, reject::custom};
use reqwest::Client;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountDataRequest {
    address: String,
    start_block: String,
    end_block: String,
}

#[derive(Debug)]
struct CustomError {
    message: &'static str,
}

impl warp::reject::Reject for CustomError {}

pub async fn get_account_data(
    body: AccountDataRequest,
    client: Client
) -> Result<impl warp::Reply, Rejection> {
    let AccountDataRequest { address, start_block, end_block } = body;

    let account_data = fetch_account_data(&client, &address, &start_block, &end_block).await;

    match account_data {
        Ok((eth_balance, token_transfers)) => {
            let response = warp::reply::json(&{(eth_balance, token_transfers)});
            Ok(warp::reply::with_status(response, StatusCode::OK))
        },
        Err(_) => Err(warp::reject::custom(CustomError { message: "Failed to fetch account data" })),
    }
}
