use crate::auth::{generate_auth_headers, Credentials};
use crate::models::{ContractResponse, PositionConfig};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

const API_BASE_URL: &str = "https://api-futures.kucoin.com";

/// A specialized HTTP client to communicate with the KuCoin Futures API.
pub struct KuCoinApiClient {
    client: Client,
    credentials: Credentials,
}

impl KuCoinApiClient {
    /// Initializes a new `KuCoinApiClient` using the provided credentials.
    pub fn new(credentials: Credentials) -> Self {
        Self {
            client: Client::new(),
            credentials,
        }
    }

    /// Fetches live contract metadata (e.g., price, multiplier, tick size) for the specified symbol.
    pub async fn fetch_contract_details(&self, symbol: &str) -> Result<ContractResponse> {
        let endpoint = format!("/api/v1/contracts/{}", symbol);
        let headers = generate_auth_headers("GET", &endpoint, "", &self.credentials)?;

        let url = format!("{}{}", API_BASE_URL, endpoint);
        let res = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .context("Failed to send request for contract details")?;

        if !res.status().is_success() {
            let error_text = res
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error text".to_string());
            anyhow::bail!("Failed to fetch contract details: {}", error_text);
        }

        let body: ContractResponse = res
            .json()
            .await
            .context("Failed to parse contract details JSON")?;
        Ok(body)
    }

    /// Executes a market entry order using isolated margin.
    pub async fn place_market_entry(
        &self,
        config: &PositionConfig,
        lots: i64,
    ) -> Result<reqwest::Response> {
        let endpoint = "/api/v1/orders";
        let body = json!({
            "clientOid": Uuid::new_v4().to_string(),
            "symbol": config.symbol,
            "side": "buy",
            "leverage": config.leverage as i64,
            "type": "market",
            "size": lots.to_string(),
            "marginMode": "ISOLATED"
        })
        .to_string();

        let headers = generate_auth_headers("POST", endpoint, &body, &self.credentials)?;
        let url = format!("{}{}", API_BASE_URL, endpoint);

        let res = self
            .client
            .post(&url)
            .headers(headers)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .context("Failed to send market entry order")?;

        Ok(res)
    }

    /// Places a limit exit order for taking profit at a target price (`closeOrder: true`).
    pub async fn place_limit_take_profit(
        &self,
        config: &PositionConfig,
        lots: i64,
        target_price: f64,
    ) -> Result<reqwest::Response> {
        let endpoint = "/api/v1/orders";
        let body = json!({
            "clientOid": Uuid::new_v4().to_string(),
            "symbol": config.symbol,
            "side": "sell",
            "leverage": config.leverage as i64,
            "type": "limit",
            "size": lots.to_string(),
            "price": format!("{:.3}", target_price),
            "closeOrder": true,
            "marginMode": "ISOLATED"
        })
        .to_string();

        let headers = generate_auth_headers("POST", endpoint, &body, &self.credentials)?;
        let url = format!("{}{}", API_BASE_URL, endpoint);

        let res = self
            .client
            .post(&url)
            .headers(headers)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .context("Failed to send take profit order")?;

        Ok(res)
    }
}
