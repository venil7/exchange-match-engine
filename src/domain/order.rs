use chrono::prelude::*;
use redis::{ErrorKind, FromRedisValue, RedisError};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
// use tracing::trace;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default, Eq, Deserialize, Serialize)]
pub enum OrderDirection {
    #[default]
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct OrderRequest {
    pub amount: i64,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub direction: OrderDirection,
}

pub fn buy_order(price: i64, amount: i64) -> OrderRequest {
    OrderRequest {
        price,
        amount,
        timestamp: chrono::Utc::now(),
        direction: OrderDirection::Buy,
    }
}
pub fn sell_order(price: i64, amount: i64) -> OrderRequest {
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
        if self.direction == other.direction {
            match self.direction {
                OrderDirection::Buy => self.price.partial_cmp(&other.price),
                OrderDirection::Sell => other.price.partial_cmp(&self.price),
            }
            .map(|direction_ordering| match direction_ordering {
                Ordering::Equal => self.timestamp.cmp(&other.timestamp),
                other => other,
            })
            .unwrap_or_else(|| self.timestamp.cmp(&other.timestamp))
        } else {
            // otherwise fallback at timestamp cmp
            self.timestamp.cmp(&other.timestamp)
        }
    }
}

// impl ToRedisArgs for OrderDirection {
//     fn write_redis_args<W>(&self, out: &mut W)
//     where
//         W: ?Sized + redis::RedisWrite,
//     {
//         match self {
//             OrderDirection::Buy => out.write_arg(b"buy"),
//             OrderDirection::Sell => out.write_arg(b"sell"),
//         }
//     }
// }

// impl ToRedisArgs for OrderRequest {
//     fn write_redis_args<W>(&self, out: &mut W)
//     where
//         W: ?Sized + redis::RedisWrite,
//     {
//         let bin = bincode::serialize(self).unwrap();
//         out.write_arg(&bin);
//         trace!("serialized {:?}", self);
//     }
// }

impl FromRedisValue for OrderRequest {
    fn from_redis_value(value: &redis::Value) -> redis::RedisResult<Self> {
        match value {
            redis::Value::Data(bytes) => {
                let order = bincode::deserialize(bytes).or_else(|_| serde_json::from_slice(bytes));
                match order {
                    Ok(order) => Ok(order),
                    _ => Err(RedisError::from((ErrorKind::IoError, "failed to decode"))),
                }
            }

            _ => Err((ErrorKind::TypeError, "cant deserialize OrderRequest").into()),
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
        let buy1 = buy_order(1, 1);
        let buy2 = buy_order(2, 1);
        assert_eq!(buy1.cmp(&buy2), Ordering::Less)
    }
    #[test]
    fn buy_orders_sorted_by_price_asc_2() {
        let buy1 = buy_order(1, 1);
        let buy2 = buy_order(2, 1);
        assert_eq!(buy2.cmp(&buy1), Ordering::Greater)
    }
    #[test]
    fn same_buy_orders_sorted_by_timestamp_correctly() {
        let buy1 = buy_order(1, 1);
        let buy2 = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..buy1
        };
        assert_eq!(buy1.cmp(&buy2), Ordering::Greater)
    }

    #[test]
    fn sell_orders_sorted_by_price_desc_1() {
        let sell1 = sell_order(1, 1);
        let sell2 = sell_order(2, 1);
        assert_eq!(sell2.cmp(&sell1), Ordering::Less)
    }
    #[test]
    fn sell_orders_sorted_by_price_desc_2() {
        let sell1 = sell_order(1, 1);
        let sell2 = sell_order(2, 1);
        assert_eq!(sell1.cmp(&sell2), Ordering::Greater)
    }
    #[test]
    fn same_sell_orders_sorted_by_timestamp_correctly() {
        let sell1 = sell_order(1, 1);
        let sell2 = OrderRequest {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..sell1
        };
        assert_eq!(sell1.cmp(&sell2), Ordering::Greater)
    }
}
