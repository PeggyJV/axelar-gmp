use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

const BASE_URL: &str = "https://api.axelarscan.io/gmp";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_alias(CamelCase, SnakeCase)]
pub struct SearchGMPRequest {
    pub tx_hash: Option<String>,
    pub from_time: Option<u64>,
    pub to_time: Option<u64>,
    pub size: Option<u32>,
    pub source_chain: Option<String>,
    pub source_address: Option<String>,
    pub destination_chain: Option<String>,
    pub contract_address: Option<String>,
}

impl SearchGMPRequest {
    pub async fn send(&self) -> Result<SearchGMPResponse, Box<dyn std::error::Error>> {
        log::debug!("sending searchGMP request {self:?}");
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/searchGMP", BASE_URL))
            .json(&self)
            .send()
            .await?;

        let response_text = response.text().await?;
        let parsed_response: SearchGMPResponse = serde_json::from_str(&response_text)?;

        if let Some(transactions) = &parsed_response.data {
            for (index, transaction) in transactions.iter().enumerate() {
                log::info!(
                    "Transaction {}: Status: {:?}, Simplified Status: {:?}",
                    index + 1,
                    transaction.status,
                    transaction.simplified_status
                );
            }
        } else {
            log::info!("No transactions found in the response");
        }

        Ok(parsed_response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_alias(CamelCase, SnakeCase)]
pub struct SearchGMPResponse {
    pub data: Option<Vec<GMPTransaction>>,
    pub total: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_alias(CamelCase, SnakeCase)]
#[serde()]
pub struct GMPTransaction {
    pub cannot_execute: Option<bool>,
    pub cannot_express_execute: Option<bool>,
    pub cannot_forecall: Option<bool>,
    pub command_id: Option<String>,
    pub error: Option<serde_json::Value>,
    pub execute_pending_transaction_hash: Option<String>,
    pub status: Option<String>,
    pub simplified_status: Option<String>,
    pub gas_status: Option<String>,
    pub source_chain: Option<String>,
    pub destination_chain: Option<String>,
    pub source_address: Option<String>,
    pub destination_address: Option<String>,
    pub source_transaction_hash: Option<String>,
    pub destination_transaction_hash: Option<String>,
}
