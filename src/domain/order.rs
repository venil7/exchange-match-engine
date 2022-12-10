use std::cmp::Ordering;

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

impl Eq for OrderRequest {}

impl Ord for OrderRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        // only compare same directions,
        if self.direction == other.direction {
            let price_compare = match self.direction {
                OrderDirection::Buy => self.price.partial_cmp(&other.price),
                OrderDirection::Sell => other.price.partial_cmp(&self.price),
            };
            match price_compare {
                Some(Ordering::Equal) => self.timestamp.cmp(&other.timestamp),
                Some(other) => other,
                _ => self.timestamp.cmp(&other.timestamp),
            }
        } else {
            // otherwise fallback at timestamp cmp
            self.timestamp.cmp(&other.timestamp)
        }
    }
}

#[cfg(test)]
mod buy_orders_sorting_tests {

    use super::{buy_order, sell_order, OrderRequest};
    use chrono::Days;
    use std::cmp::Ordering;

    #[test]
    fn buy_orders_sorted_by_price_asc_1() {
        let buy1 = buy_order(1., 1.);
        let buy2 = buy_order(2., 1.);
        assert_eq!(buy1.cmp(&buy2), Ordering::Less)
    }
    #[test]
    fn buy_orders_sorted_by_price_asc_2() {
        let buy1 = buy_order(1., 1.);
        let buy2 = buy_order(2., 1.);
        assert_eq!(buy2.cmp(&buy1), Ordering::Greater)
    }
    #[test]
    fn same_buy_orders_sorted_by_timestamp_correctly() {
        let buy1 = buy_order(1., 1.);
        let buy2 = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..buy1
        };
        assert_eq!(buy1.cmp(&buy2), Ordering::Greater)
    }

    #[test]
    fn sell_orders_sorted_by_price_desc_1() {
        let sell1 = sell_order(1., 1.);
        let sell2 = sell_order(2., 1.);
        assert_eq!(sell2.cmp(&sell1), Ordering::Less)
    }
    #[test]
    fn sell_orders_sorted_by_price_desc_2() {
        let sell1 = sell_order(1., 1.);
        let sell2 = sell_order(2., 1.);
        assert_eq!(sell1.cmp(&sell2), Ordering::Greater)
    }
    #[test]
    fn same_sell_orders_sorted_by_timestamp_correctly() {
        let sell1 = sell_order(1., 1.);
        let sell2 = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..sell1
        };
        assert_eq!(sell1.cmp(&sell2), Ordering::Greater)
    }
}
