use crate::order::{OrderDirection, OrderRequest};
use std::cmp::Ordering;

impl Eq for OrderRequest {}

impl Ord for OrderRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self.direction == (&other.direction) {
            // only compare same directions,
            true => match self.direction {
                OrderDirection::Buy => self
                    .amount
                    .partial_cmp(&other.amount)
                    .or_else(|| Some(self.timestamp.cmp(&other.timestamp)))
                    .unwrap_or(Ordering::Equal),
                OrderDirection::Sell => other
                    .amount
                    .partial_cmp(&self.amount)
                    .or_else(|| Some(self.timestamp.cmp(&other.timestamp)))
                    .unwrap_or(Ordering::Equal),
            },
            // otherwise fallback at timestamp cmp
            _ => return self.timestamp.cmp(&other.timestamp),
        }
    }
}

#[cfg(test)]
mod tests {

    use sorted_vec::SortedVec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_buy_in_order() {
        let buy1 = OrderRequest {
            direction: OrderDirection::Buy,
            amount: 1.,
            ..Default::default()
        };
        let buy2 = OrderRequest {
            direction: OrderDirection::Buy,
            amount: 2.,
            ..Default::default()
        };

        let buys = SortedVec::from(vec![buy1, buy2]);
        assert_eq!(buys.first(), Some(&buy1))
    }
}
