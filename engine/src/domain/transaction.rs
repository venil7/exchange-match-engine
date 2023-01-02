use chrono::Utc;
use redis::ToRedisArgs;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use super::Order;
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tx {
    pub id: Uuid,
    pub lhs: Order,
    pub rhs: Order,
    pub timestamp: chrono::DateTime<Utc>,
}

impl Tx {
    pub fn new(lhs: Order, rhs: Order) -> Tx {
        Tx {
            lhs,
            rhs,
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl ToRedisArgs for Tx {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match serde_json::to_string(&self) {
            Ok(str) => out.write_arg(&str.as_bytes()),
            _ => (),
        }
    }
}

impl Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{id}: {lhs} <-> {rhs}",
            id = self.id,
            lhs = self.lhs,
            rhs = self.rhs
        ))
    }
}
