use chrono::prelude::*;
use redis::{ErrorKind, FromRedisValue, RedisError, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Default, Eq, Deserialize, Serialize)]
pub enum OrderState {
    #[default]
    Pending,
    Partial,
    Complete,
    Cancelled,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Default, Eq, Deserialize, Serialize)]
pub enum OrderDirection {
    #[default]
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, Hash, Default, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct OrderRequest {
    pub amount: i64,
    pub price: i64,
    pub direction: OrderDirection,
}

impl From<OrderRequest> for Order {
    fn from(value: OrderRequest) -> Self {
        match value.direction {
            OrderDirection::Buy => buy_order(value.price, value.amount),
            _ => sell_order(value.price, value.amount),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Default, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub amount: i64,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub state: OrderState,
    pub direction: OrderDirection,
}

impl ToRedisArgs for Order {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        if let Ok(json) = serde_json::to_string(self) {
            out.write_arg(json.as_bytes());
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{direction:?} {amount}@{price}",
            direction = self.direction,
            amount = self.amount,
            price = self.price
        ))
    }
}

pub fn buy_order(price: i64, amount: i64) -> Order {
    Order {
        price,
        amount,
        id: Uuid::new_v4(),
        state: OrderState::Pending,
        timestamp: chrono::Utc::now(),
        direction: OrderDirection::Buy,
    }
}
pub fn sell_order(price: i64, amount: i64) -> Order {
    Order {
        price,
        amount,
        id: Uuid::new_v4(),
        state: OrderState::Pending,
        timestamp: chrono::Utc::now(),
        direction: OrderDirection::Sell,
    }
}
impl Order {
    pub fn with_state(&self, state: OrderState) -> Self {
        Order {
            state,
            ..self.clone()
        }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.direction == other.direction {
            return match self.direction {
                OrderDirection::Buy => self.price.cmp(&other.price),
                OrderDirection::Sell => other.price.cmp(&self.price),
            }
            .then(self.timestamp.cmp(&other.timestamp))
            .then(self.amount.cmp(&other.amount))
            .then(self.id.cmp(&other.id));
        }
        // otherwise fallback at timestamp cmp
        self.id
            .cmp(&other.id)
            .then(self.timestamp.cmp(&other.timestamp))
            .then(self.price.cmp(&other.price))
            .then(self.amount.cmp(&other.amount))
    }
}

impl FromRedisValue for Order {
    fn from_redis_value(value: &redis::Value) -> redis::RedisResult<Self> {
        match value {
            redis::Value::Data(bytes) => {
                let order = bincode::deserialize(bytes).or_else(|_| serde_json::from_slice(bytes));
                match order {
                    Ok(order) => Ok(order),
                    _ => Err(RedisError::from((ErrorKind::IoError, "failed to decode"))),
                }
            }

            _ => Err((ErrorKind::TypeError, "cant deserialize Order").into()),
        }
    }
}

#[cfg(test)]
mod buy_orders_sorting_tests {

    use super::{buy_order, sell_order, Order};
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
        let buy2 = Order {
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
        let sell2 = Order {
            timestamp: chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap(),
            ..sell1
        };
        assert_eq!(sell1.cmp(&sell2), Ordering::Greater)
    }
}
