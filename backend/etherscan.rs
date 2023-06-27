use serde::Deserialize;
use serde_json::Value;
use reqwest::Client;

const ETHERSCAN_API: &str = "https://api.etherscan.io/api";

#[derive(Deserialize)]
struct EtherscanResponse<T> {
    result: T,
}

pub async fn fetch_account_data(
    client: &Client,
    address: &str,
    start_block: &str,
    end_block: &str,
) -> Result<(Value, Value), Box<dyn std::error::Error>> {
    // Fetch ETH balance
    let params = [
        ("module", "account"),
        ("action", "balance"),
        ("address", address),
        ("tag", "latest"),
        ("startblock", start_block),
        ("endblock", end_block),
        ("apikey", "A5FYI2UF4KAZUE929VGP7QIVNWAIC88XVC"), // this the actual API key
    ];

    let resp = client
        .get(ETHERSCAN_API)
        .query(&params)
        .send()
        .await?
        .json::<EtherscanResponse<Value>>()
        .await?;

    let eth_balance = resp.result;

    // Fetch ERC20 token transfers
    let params = [
        ("module", "account"),
        ("action", "tokentx"),
        ("address", address),
        ("startblock", start_block),
        ("endblock", end_block),
        ("sort", "asc"),
        ("apikey", "A5FYI2UF4KAZUE929VGP7QIVNWAIC88XVC"), //  this the actual API key
    ];

    let resp = client
        .get(ETHERSCAN_API)
        .query(&params)
        .send()
        .await?
        .json::<EtherscanResponse<Value>>()
        .await?;

    let token_transfers = resp.result;

    Ok((eth_balance, token_transfers))
}
