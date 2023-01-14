use anyhow::Result;
use domain::{Order, OrderBook, OrderRequest, Tx};
use uuid::Uuid;

pub trait OrderBookProvider {
    fn enqueue(&mut self, order_request: OrderRequest) -> Result<Order>;
    fn dequeue(&mut self) -> Result<Order>;

    fn save_order_book(&mut self, book: &OrderBook) -> Result<()>;
    fn load_order_book(&mut self) -> Result<OrderBook>;

    fn get_order(&mut self, id: &Uuid) -> Result<Order>;
    fn set_order(&mut self, order: Order) -> Result<()>;

    fn mark_processed(&mut self, txs: &[Tx]) -> Result<()>;
}
