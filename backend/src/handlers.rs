use crate::etherscan::fetch_account_data;
use warp::{http::StatusCode, Rejection, Reply, reject::custom};
use reqwest::Client;
use std::env;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountDataRequest {
    address: String,
    start_block: String,
    end_block: String,
    date: String, // Added to handle the new date filter
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
    let AccountDataRequest { address, start_block, end_block, date } = body;

    // Parse the date string into a NaiveDate
    let date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(dt) => dt,
        Err(e) => {
            return Err(warp::reject::custom(CustomError { message: "Invalid date format" }));
        }
    };

    let account_data = fetch_account_data(&client, &address, &start_block, &end_block, &date).await;

    match account_data {
        Ok((eth_balance, token_transfers, eth_transfers)) => {
            let response = warp::reply::json(&{ (eth_balance, token_transfers, eth_transfers) });
            Ok(warp::reply::with_status(response, StatusCode::OK))
        },
        Err(_) => Err(warp::reject::custom(CustomError { message: "Failed to fetch account data" })),
    }
}
