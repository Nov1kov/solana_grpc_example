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
pub struct ShyftResponse<T> {
    pub success: bool,
    pub message: String,
    pub result: Option<T>,
}

pub struct ShyftClient {
    client: Client,
    api_key: String,
    network: String,
}

impl ShyftClient {
    pub fn new(api_key: &str, network: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            network: network.to_string(),
        }
    }

    /// Отправляет подписанную транзакцию через Shyft API
    pub async fn send_transaction(&self, encoded_transaction: &str) -> Result<TxnResult, Box<dyn std::error::Error>> {
        let request = SendTxnRequest {
            network: self.network.clone(),
            encoded_transaction: encoded_transaction.to_string(),
        };

        let response = self.client
            .post("https://api.shyft.to/sol/v1/transaction/send_txn")
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json::<ShyftResponse<TxnResult>>()
            .await?;

        if !response.success {
            return Err(format!("Ошибка отправки транзакции: {}", response.message).into());
        }

        response.result.ok_or_else(|| "Отсутствует результат".into())
    }

    /// Получает информацию о балансе адреса
    pub async fn get_balance(&self, address: String) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://api.shyft.to/sol/v1/wallet/balance")
            .header("x-api-key", &self.api_key)
            .query(&[
                ("network", &self.network),
                ("wallet", &address)
            ])
            .send()
            .await?
            .json::<ShyftResponse<serde_json::Value>>()
            .await?;

        if !response.success {
            return Err(format!("Ошибка получения баланса: {}", response.message).into());
        }

        let balance = response.result
            .ok_or("Отсутствует результат")?
            .get("balance")
            .ok_or("Отсутствует поле balance в ответе")?
            .as_f64()
            .ok_or("Невозможно преобразовать баланс в число")?;

        // Преобразуем SOL в лэмпорты
        let lamports = (balance * 1_000_000_000.0) as u64;
        Ok(lamports)
    }
}