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
