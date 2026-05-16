use serde::Deserialize;

/// Represents the raw contract data received from KuCoin's API.
#[derive(Debug, Deserialize)]
pub struct ContractData {
    /// The most recent price the contract traded at.
    #[serde(rename = "lastTradePrice")]
    pub last_trade_price: f64,
    /// The contract multiplier used to convert between contract lots and underlying asset.
    pub multiplier: f64,
    /// The minimum price change (tick size) allowed for the contract.
    #[serde(rename = "tickSize")]
    pub tick_size: f64,
}

/// A wrapper for the successful response payload containing contract data.
#[derive(Debug, Deserialize)]
pub struct ContractResponse {
    pub data: ContractData,
}

/// Configuration settings for calculating position sizes and targets.
#[derive(Debug)]
pub struct PositionConfig {
    /// The capital allocated to this trade in USDT.
    pub margin_usdt: f64,
    /// The leverage applied to the position.
    pub leverage: f64,
    /// The desired profit target in USDT.
    pub profit_target_usdt: f64,
    /// The trading pair symbol (e.g., "SOLUSDTM").
    pub symbol: String,
}

/// The result of calculating order parameters from live market data.
#[derive(Debug)]
pub struct PositionCalculation {
    /// The total value of the position in USDT (Margin * Leverage).
    pub position_value_usdt: f64,
    /// The calculated number of contracts (lots) to trade.
    pub lots: i64,
    /// The exact target price for the take-profit order, rounded to the tick size.
    pub target_tp_price: f64,
}
