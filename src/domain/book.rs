use super::{order::OrderRequest, OrderDirection, Spread, Tx};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};
use tracing::{info, trace};

pub type PricePoint = BTreeSet<OrderRequest>;
pub type OrderBookSide = BTreeMap<i64, PricePoint>;

pub trait OrderBookSideImpl {
    fn to_string(&self) -> String;
}
pub trait PricePointImpl {
    fn total_amount(&self) -> i64;
    fn consume(&mut self, other: &mut Self) -> Vec<Tx>;
}
impl OrderBookSideImpl for OrderBookSide {
    fn to_string(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{amount}@{price},", amount = v.total_amount(), price = k))
            .collect()
    }
}
impl PricePointImpl for PricePoint {
    fn total_amount(&self) -> i64 {
        self.iter()
            .fold(0, |acc, order: &OrderRequest| acc + order.amount)
    }

    // lhs is assumed to be bigger in total amount
    // than rhs when this fn is called
    // returns txs for consumed orders
    fn consume(&mut self, other: &mut Self) -> Vec<Tx> {
        let mut txs = vec![];
        while !self.is_empty() && !other.is_empty() {
            let mut lhs = self.pop_first().unwrap();
            let mut rhs = other.pop_first().unwrap();
            match lhs.amount.cmp(&rhs.amount) {
                Ordering::Equal => txs.push(Tx::new(lhs, rhs)),
                Ordering::Less => {
                    rhs.amount -= lhs.amount;
                    txs.push(Tx::new(
                        lhs,
                        OrderRequest {
                            amount: lhs.amount,
                            ..rhs
                        },
                    ));
                    other.insert(rhs);
                }
                Ordering::Greater => {
                    lhs.amount -= rhs.amount;
                    txs.push(Tx::new(
                        OrderRequest {
                            amount: rhs.amount,
                            ..lhs
                        },
                        rhs,
                    ));
                    self.insert(lhs);
                }
            }
        }

        txs
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
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

    pub fn report(&self) {
        info!(
            "order book:\nbuys:{buys}\nsells:{sells}",
            buys = self.buys.to_string(),
            sells = self.sells.to_string(),
        );
    }

    pub fn spread(&self) -> Spread {
        let b = self.buys.last_key_value().map(|(&x, _)| x);
        let s = self.sells.first_key_value().map(|(&x, _)| x);
        Spread(b, s)
    }

    // returns processed orders
    pub fn match_and_process_orders(&mut self) -> Vec<Tx> {
        let mut txs = vec![];
        let mut spread = self.spread();
        while spread.overlaping() {
            let (price, mut buy) = self.buys.pop_last().unwrap();
            let (_, mut sell) = self.sells.pop_last().unwrap();
            trace!(
                "transacting at {price}, buys total {buys_total}, sells total {sells_total}",
                price = price,
                buys_total = buy.total_amount(),
                sells_total = sell.total_amount()
            );

            match buy.total_amount().cmp(&sell.total_amount()) {
                Ordering::Equal => txs.append(&mut buy.consume(&mut sell)),
                //theres more buy orders, then sell
                Ordering::Greater => {
                    txs.append(&mut buy.consume(&mut sell));
                    self.buys.insert(price, buy);
                }
                //theres more sell orders, then buy
                Ordering::Less => {
                    txs.append(&mut sell.consume(&mut buy));
                    self.sells.insert(price, sell);
                }
            }
            spread = self.spread();
        }
        txs
    }

    pub fn add_order(&mut self, order: OrderRequest) {
        match order.direction {
            OrderDirection::Buy => self
                .buys
                .entry(order.price)
                .or_insert_with(BTreeSet::new)
                .insert(order),
            OrderDirection::Sell => self
                .sells
                .entry(order.price)
                .or_insert_with(BTreeSet::new)
                .insert(order),
        };
    }
}

#[cfg(test)]
mod order_book_tests {

    use super::OrderBook;
    use crate::domain::{buy_order, sell_order, OrderRequest, PricePointImpl, Spread};
    use chrono::Days;
    use uuid::Uuid;

    #[test]
    fn order_total_amount_test() {
        let buy1 = buy_order(1, 3);
        let buy2 = OrderRequest {
            id: Uuid::new_v4(),
            amount: 2,
            ..buy1
        };

        let mut book: OrderBook = Default::default();
        book.add_order(buy1);
        book.add_order(buy2);

        let total = book.buys.pop_first().unwrap().1.total_amount();
        assert_eq!(total, buy1.amount + buy2.amount);
    }

    #[test]
    fn orderbook_nonoverlaping_spread_test() {
        let buy1 = buy_order(1, 1);
        let buy2 = buy_order(2, 1);

        let sell1 = sell_order(3, 1);
        let sell2 = sell_order(4, 1);

        let mut book: OrderBook = Default::default();
        book.add_order(buy1);
        book.add_order(buy2);
        book.add_order(sell1);
        book.add_order(sell2);

        let spread = book.spread();
        assert_eq!(spread, Spread(Some(2), Some(3)));
        assert_eq!(spread.overlaping(), false);
    }

    #[test]
    fn buy_orders_ordered_asc() {
        let buy1 = buy_order(1, 1);
        let buy2 = buy_order(3, 1);

        let sell1 = sell_order(2, 1);
        let sell2 = sell_order(4, 1);

        let mut book: OrderBook = Default::default();
        book.add_order(buy1);
        book.add_order(buy2);
        book.add_order(sell1);
        book.add_order(sell2);

        let spread = book.spread();
        assert_eq!(spread, Spread(Some(3), Some(2)));
        assert_eq!(spread.overlaping(), true);
    }

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
        book.add_order(oldest_sell);
        book.add_order(recent_sell);
        book.add_order(not_so_old_sell);

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
        book.add_order(oldest_buy);
        book.add_order(recent_buy);
        book.add_order(not_so_old_buy);

        let (price, mut sell_orders) = book.buys.pop_first().unwrap();
        let first_sell_order = sell_orders.pop_first().unwrap();

        assert_eq!(price, 1);
        assert_eq!(first_sell_order, oldest_buy);
    }
}
