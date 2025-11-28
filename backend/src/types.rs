use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PricePoint {
    pub symbol: String,
    pub price_scaled: i64,
    pub source: String,
    pub confidence: i64,
    pub expo: i32,
    pub timestamp: DateTime<Utc>,
    pub is_fallback: bool,
}
