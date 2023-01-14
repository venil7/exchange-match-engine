use anyhow::anyhow;
use anyhow::Result;
use domain::{Order, OrderBook, OrderRequest, Tx};
use r2d2::PooledConnection;
use redis::{Commands, RedisError};
use std::ops::DerefMut;
use tracing::info;

use crate::OrderBookProvider;

pub struct RedisProvider<C: DerefMut<Target = redis::Connection> + ?Sized> {
    ticker: String,
    connection: C,
}

impl RedisProvider<Box<redis::Connection>> {
    pub fn new(ticker: &str, url: &str) -> Result<Self> {
        let connection = redis::Client::open(url)?.get_connection()?;
        // info!("Connected to {url}", url = url);
        Ok(Self {
            ticker: ticker.into(),
            connection: Box::new(connection),
        })
    }
}
impl RedisProvider<PooledConnection<redis::Client>> {
    pub fn try_from(
        ticker: &str,
        connection: Option<PooledConnection<redis::Client>>,
    ) -> Option<Self> {
        connection.map(|connection| Self {
            ticker: ticker.into(),
            connection,
        })
    }
}

impl<C: DerefMut<Target = redis::Connection>> OrderBookProvider for RedisProvider<C> {
    fn enqueue(&mut self, order_request: OrderRequest) -> Result<Order> {
        let key = format!("{ticker}/orders", ticker = self.ticker);
        let order: Order = order_request.into();
        let payload: Result<i32, RedisError> = self.connection.rpush(key, order);
        match payload {
            Ok(_) => Ok(order),
            Err(err) => Err(anyhow::anyhow!("not valid order: {err}", err = err)),
        }
    }

    fn dequeue(&mut self) -> Result<Order> {
        let key = format!("{ticker}/orders", ticker = self.ticker);
        let payload: Result<(String, Order), RedisError> = self.connection.blpop(key, 0);
        match payload {
            Ok((_, order)) => Ok(order),
            Err(err) => Err(anyhow::anyhow!("not valid order: {err}", err = err)),
        }
    }

    fn save_order_book(&mut self, book: &OrderBook) -> Result<()> {
        let bytes = bincode::serialize(book).unwrap();
        self.connection
            .set(format!("{ticker}/order_book", ticker = self.ticker), &bytes)?;
        book.report();
        Ok(())
    }

    fn load_order_book(&mut self) -> Result<OrderBook> {
        let bytes: Vec<u8> = self
            .connection
            .get(format!("{ticker}/order_book", ticker = self.ticker))?;
        let book: OrderBook = bincode::deserialize(&bytes).unwrap_or_default();
        book.report();
        Ok(book)
    }

    fn mark_processed(&mut self, txs: &[Tx]) -> Result<()> {
        let key = &format!("{ticker}/txs", ticker = self.ticker);
        for tx in txs {
            info!("processed: {tx}", tx = tx);
            self.connection.rpush(key, tx)?;
        }
        Ok(())
    }

    fn get_order(&mut self, id: &uuid::Uuid) -> Result<Order> {
        let key = &format!("{ticker}/{uuid}", ticker = self.ticker, uuid = id);
        let order = self.connection.get(key)?;
        Ok(order)
    }

    fn set_order(&mut self, order: Order) -> Result<()> {
        let key = &format!("{ticker}/{uuid}", ticker = self.ticker, uuid = order.id);
        // let bytes = serde_json::to_string(&order)?;
        self.connection.set(key, order)?;
        Ok(())
    }
}
