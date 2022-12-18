use super::{order::OrderRequest, OrderDirection};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type OrderBookSide = BTreeMap<i64, OrderRequest>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OrderBook {
    pub buys: OrderBookSide,
    pub sells: OrderBookSide,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn match_order(&mut self, order: OrderRequest) -> Option<Vec<OrderRequest>> {
        match order.direction {
            OrderDirection::Buy => self.buys.insert(order.price, order),
            OrderDirection::Sell => self.sells.insert(order.price, order),
        };
        None
    }
}
