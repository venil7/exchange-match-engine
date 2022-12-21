use std::fmt::Display;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::OrderRequest;
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tx {
    pub id: Uuid,
    pub lhs: OrderRequest,
    pub rhs: OrderRequest,
    pub timestamp: chrono::DateTime<Utc>,
}

impl Tx {
    pub fn new(lhs: OrderRequest, rhs: OrderRequest) -> Tx {
        Tx {
            id: Uuid::new_v4(),
            lhs,
            rhs,
            timestamp: chrono::Utc::now(),
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
