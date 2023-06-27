use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccountData {
    pub address: String,
    pub balance: String,
    pub transactions: Vec<Transaction>,
}
