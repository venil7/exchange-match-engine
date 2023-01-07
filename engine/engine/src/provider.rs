use anyhow::Result;
use async_trait::async_trait;
use domain::{Order, OrderBook, OrderRequest, Tx};
use redis::{aio::AsyncStream, AsyncCommands, Client, RedisError};
use std::pin::Pin;
use tracing::info;

pub enum OrdersOps {
    Request,
    //
}

#[async_trait]
pub trait OrderBookProvider {
    async fn enqueue(&mut self, order_request: OrderRequest) -> Result<Order>;
    async fn dequeue(&mut self) -> Result<Order>;

    async fn save_order_book(&mut self, book: &OrderBook) -> Result<()>;
    async fn load_order_book(&mut self) -> Result<OrderBook>;

    // async fn get_order(&mut self, txs: &[Tx]) -> Result<()>;
    async fn mark_processed(&mut self, txs: &[Tx]) -> Result<()>;
}

pub struct RedisProvider {
    ticker: String,
    connection: redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
}

impl RedisProvider {
    pub async fn try_new(ticker: &str, url: &str) -> Result<Self> {
        let connection = Client::open(url)?.get_async_connection().await?;
        info!("Connected to {url}", url = url);
        Ok(Self {
            ticker: ticker.into(),
            connection,
        })
    }
}

#[async_trait]
impl OrderBookProvider for RedisProvider {
    async fn enqueue(&mut self, order_request: OrderRequest) -> Result<Order> {
        let key = format!("{ticker}/orders", ticker = self.ticker);
        let order: Order = order_request.into();
        let payload: Result<i32, RedisError> = self.connection.rpush(key, order).await;
        match payload {
            Ok(_) => Ok(order),
            Err(err) => Err(anyhow::anyhow!("not valid order: {err}", err = err)),
        }
    }

    async fn dequeue(&mut self) -> Result<Order> {
        let key = format!("{ticker}/orders", ticker = self.ticker);
        let payload: Result<(String, Order), RedisError> = self.connection.blpop(key, 0).await;
        match payload {
            Ok((_, order)) => Ok(order),
            Err(err) => Err(anyhow::anyhow!("not valid order: {err}", err = err)),
        }
    }

    async fn save_order_book(&mut self, book: &OrderBook) -> Result<()> {
        let bytes = bincode::serialize(book).unwrap();
        self.connection
            .set(format!("{ticker}/order_book", ticker = self.ticker), &bytes)
            .await?;
        book.report();
        Ok(())
    }

    async fn load_order_book(&mut self) -> Result<OrderBook> {
        let bytes: Vec<u8> = self
            .connection
            .get(format!("{ticker}/order_book", ticker = self.ticker))
            .await?;
        let book: OrderBook = bincode::deserialize(&bytes).unwrap_or_default();
        book.report();
        Ok(book)
    }

    async fn mark_processed(&mut self, txs: &[Tx]) -> Result<()> {
        let key = &format!("{ticker}/txs", ticker = self.ticker);
        for tx in txs {
            info!("processed: {tx}", tx = tx);
            self.connection.rpush(key, tx).await?;
        }
        Ok(())
    }
}
