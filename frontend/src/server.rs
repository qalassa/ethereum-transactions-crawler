use std::collections::HashMap;
use warp::Filter;
use warp::Rejection;
use serde::Deserialize;

use crate::service::get_address_info;

#[derive(Debug, Deserialize)]
pub struct FormData {
    address: String,
    start_block: String,
    end_block: String,
    date: String, // Added to handle the new date filter
}


pub async fn form_handler(form_data: HashMap<String, String>) -> Result<impl warp::Reply, Rejection> {
    println!("Received form data: {:?}", form_data);  // Debug print statement
    let address = form_data.get("address").cloned().unwrap_or_default();
    let start_block = form_data.get("start_block").cloned().unwrap_or_default();
    let end_block = form_data.get("end_block").cloned().unwrap_or_default();
    let date = form_data.get("date").cloned().unwrap_or_default();  // Add this line

    let response = get_address_info(&address, &start_block, &end_block, &date).await.unwrap_or_default();  // Update this line

    Ok(warp::reply::json(&response))
}

pub fn submit_form() -> impl Filter<Extract = (HashMap<String, String>,), Error = Rejection> + Clone {
    warp::path!("submit_form")
        .and(warp::post())
        .and(warp::body::form())
}

pub fn form() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .map(|| "Submit form data here")
}
