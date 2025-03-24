use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize)]
pub struct SendTxnRequest {
    pub network: String,
    pub encoded_transaction: String,
}

#[derive(Deserialize, Debug)]
pub struct TxnResult {
    pub signature: String,
}

#[derive(Deserialize, Debug)]
pub struct BalanceResult {
    pub balance: f64,
}

#[derive(Deserialize, Debug)]
pub struct ShyftResponse<T> {
    pub success: bool,
    pub message: String,
    pub result: Option<T>,
}

pub struct ShyftClient {
    client: Client,
    api_key: String,
    network: String,
    base_url: String,
}

impl ShyftClient {
    pub fn new(base_url: &str, api_key: &str, network: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            network: network.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn send_transaction(&self, encoded_transaction: String) -> Result<TxnResult, Box<dyn std::error::Error>> {
        let request = SendTxnRequest {
            network: self.network.clone(),
            encoded_transaction,
        };

        let response = self.client
            .post(format!("{}/sol/v1/transaction/send_txn", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let shyft_response = response.json::<ShyftResponse<TxnResult>>().await?;
            if shyft_response.success && shyft_response.result.is_some() {
                Ok(shyft_response.result.unwrap())
            } else {
                Err(format!("Parsing error: {}", shyft_response.message).into())
            }
        } else {
            Err(format!("Response error: {}", response.text().await?).into())
        }
    }

    pub async fn get_balance(&self, address: String) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self.client
            .get(format!("{}/sol/v1/wallet/balance", self.base_url))
            .header("x-api-key", &self.api_key)
            .query(&[
                ("network", &self.network),
                ("wallet", &address)
            ])
            .send()
            .await?
            .json::<ShyftResponse<BalanceResult>>()
            .await?;

        if !response.success {
            return Err(format!("Ошибка получения баланса: {}", response.message).into());
        }

        if let Some(balance) = response.result {
            // Преобразуем SOL в лэмпорты
            let lamports = (balance.balance * 1_000_000_000.0) as u64;
            Ok(lamports)
        } else {
            Err(format!("Error parsing balance: {}", response.message).into())
        }
    }
}