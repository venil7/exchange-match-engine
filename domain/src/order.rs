use chrono::prelude::*;

#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Default, Eq, serde::Deserialize, serde::Serialize,
)]
pub enum OrderDirection {
    #[default]
    Buy,
    Sell,
}

#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Default, serde::Deserialize, serde::Serialize,
)]
pub struct OrderRequest {
    pub amount: f64,
    pub price: f64,
    pub timestamp: chrono::DateTime<Utc>,
    pub direction: OrderDirection,
}

pub fn buy_order(price: f64, amount: f64) -> OrderRequest {
    OrderRequest {
        price,
        amount,
        timestamp: chrono::Utc::now(),
        direction: OrderDirection::Buy,
    }
}
pub fn sell_order(price: f64, amount: f64) -> OrderRequest {
    OrderRequest {
        price,
        amount,
        timestamp: chrono::Utc::now(),
        direction: OrderDirection::Sell,
    }
}
