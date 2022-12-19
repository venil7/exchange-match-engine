use super::{order::OrderRequest, OrderDirection};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use tracing::info;

pub type OrderBookSide = BTreeMap<i64, BTreeSet<OrderRequest>>;

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

    fn report(&self) {
        info!(
            "order book: buys:{buys}, sells:{sells}",
            buys = self.buys.len(),
            sells = self.sells.len(),
        );
    }

    pub fn match_order(&mut self, order: OrderRequest) -> Option<Vec<OrderRequest>> {
        match order.direction {
            OrderDirection::Buy => self.buys.entry(order.price).or_default().insert(order),
            OrderDirection::Sell => self.sells.entry(order.price).or_default().insert(order),
        };
        self.report();
        None
    }
}

#[cfg(test)]
mod order_book_tests {

    use super::OrderBook;
    use crate::domain::{buy_order, sell_order, OrderRequest};
    use chrono::Days;

    #[test]
    fn sell_orders_of_the_same_rank_appear_historically() {
        let recent_sell = sell_order(1, 1);
        let not_so_old_sell = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..recent_sell
        };
        let oldest_sell = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(2)).unwrap(),
            ..recent_sell
        };
        let mut book: OrderBook = Default::default();
        book.match_order(oldest_sell);
        book.match_order(recent_sell);
        book.match_order(not_so_old_sell);

        let (price, mut sell_orders) = book.sells.pop_first().unwrap();
        let first_sell_order = sell_orders.pop_first().unwrap();

        assert_eq!(price, 1);
        assert_eq!(first_sell_order, oldest_sell);
    }

    #[test]
    fn buy_orders_of_the_same_rank_appear_historically() {
        let recent_buy = buy_order(1, 1);
        let not_so_old_buy = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..recent_buy
        };
        let oldest_buy = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(2)).unwrap(),
            ..recent_buy
        };
        let mut book: OrderBook = Default::default();
        book.match_order(oldest_buy);
        book.match_order(recent_buy);
        book.match_order(not_so_old_buy);

        let (price, mut sell_orders) = book.buys.pop_first().unwrap();
        let first_sell_order = sell_orders.pop_first().unwrap();

        assert_eq!(price, 1);
        assert_eq!(first_sell_order, oldest_buy);
    }
}
