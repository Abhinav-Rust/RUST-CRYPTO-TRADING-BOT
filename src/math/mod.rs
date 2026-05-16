use crate::models::{PositionCalculation, PositionConfig};

/// Dynamically calculates the exact position size (in lots) and take-profit exit price.
///
/// It uses the `last_trade_price` and `multiplier` to convert the leveraged USDT position value
/// into contract lots, ensuring proper integer bounds. It then calculates the necessary price
/// shift (`tp_price_diff`) for the target profit, ensuring the result is properly aligned with
/// the required `tick_size` step.
pub fn calculate_position(
    config: &PositionConfig,
    last_trade_price: f64,
    multiplier: f64,
    tick_size: f64,
) -> Option<PositionCalculation> {
    let position_value_usdt = config.margin_usdt * config.leverage;
    let position_size_sol = position_value_usdt / last_trade_price;

    // Calculate raw lots (contract size)
    let raw_lots = position_size_sol / multiplier;

    // Round to nearest integer lot size
    let lots = raw_lots.round() as i64;

    if lots <= 0 {
        return None;
    }

    // Calculate target take profit price
    // Profit = (TP - Entry) * (Lots * Multiplier)
    let tp_price_diff = config.profit_target_usdt / (lots as f64 * multiplier);
    let target_tp_price = last_trade_price + tp_price_diff;

    // Round to nearest tick_size
    let rounded_tp_price = (target_tp_price / tick_size).round() * tick_size;

    Some(PositionCalculation {
        position_value_usdt,
        lots,
        target_tp_price: rounded_tp_price,
    })
}

#[cfg(test)]
mod tests;
