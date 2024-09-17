use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

const BASE_URL: &str = "https://api.axelarscan.io/gmp";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        log::debug!(request = self; "sending searchGMP request");
        log::debug!("request json: {:?}", serde_json::to_string_pretty(&self)?);
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/searchGMP", BASE_URL))
            .json(&self)
            .send()
            .await?;

        let response_text = response.text().await?;
        log::debug!(response = response_text; "searchGMP response");

        // Write response to JSON file
        let file_name = "searchGMP_response.json";
        let mut file = std::fs::File::create(file_name)?;
        std::io::Write::write_all(&mut file, response_text.as_bytes())?;
        log::debug!("Response written to file: {file_name}");

        let parsed_response: SearchGMPResponse = serde_json::from_str(&response_text)?;

        if let Some(transactions) = &parsed_response.data {
            for (index, transaction) in transactions.iter().enumerate() {
                log::debug!("Transaction {}: {:?}", index + 1, transaction);
            }
        } else {
            log::debug!("No transactions found in the response");
        }

        Ok(parsed_response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGMPResponse {
    pub data: Option<Vec<GMPTransaction>>,
    pub total: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GMPTransaction {
    pub call: Call,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub chain: String,
    pub axelar_transaction_hash: String,
    pub transaction_hash: String,
    pub event: String,
}
