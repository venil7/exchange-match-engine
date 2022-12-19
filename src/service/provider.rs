use anyhow::Result;
use async_trait::async_trait;
use redis::{aio::AsyncStream, AsyncCommands, Client, RedisError};
use std::pin::Pin;
use tracing::{info, trace};

use crate::domain::{OrderBook, OrderRequest};

#[async_trait]
pub trait OrderProvider {
    async fn next_order(&mut self) -> Result<OrderRequest>;
    async fn save_order_book(&mut self, book: &OrderBook) -> Result<()>;
    async fn load_order_book(&mut self) -> Result<OrderBook>;
    async fn mark_processed(&mut self, _orders: &[OrderRequest]) -> Result<()>;
}

pub struct RedisProvider {
    pair: String,
    connection: redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
}

impl RedisProvider {
    pub async fn try_new(pair: &str, url: &str) -> Result<Self> {
        let connection = Client::open(url)?.get_async_connection().await?;
        info!("Connected to {url}", url = url);
        Ok(Self {
            pair: pair.into(),
            connection,
        })
    }
}

#[async_trait]
impl OrderProvider for RedisProvider {
    async fn next_order(&mut self) -> Result<OrderRequest> {
        let key = format!("{pair}/orders", pair = self.pair);
        info!("send orders to {queue}", queue = key);
        let payload: Result<(String, OrderRequest), RedisError> =
            self.connection.blpop(key, 0).await;
        match payload {
            Ok((_, order)) => Ok(order),
            Err(err) => Err(anyhow::anyhow!("not valid order: {err}", err = err)),
        }
    }

    async fn save_order_book(&mut self, book: &OrderBook) -> Result<()> {
        trace!("saving {:?}", book);
        // self.connection
        //     .set(format!("{pair}/buys", pair = self.pair), &book.buys)
        //     .await?;
        // self.connection
        //     .set(format!("{pair}/sells", pair = self.pair), &book.sells)
        //     .await?;

        Ok(())
    }

    async fn load_order_book(&mut self) -> Result<OrderBook> {
        // let buys: Option<OrderBookSide> = self
        //     .connection
        //     .get(format!("{pair}/buys", pair = self.pair))
        //     .await?;

        // let sells: Option<OrderBookSide> = self
        //     .connection
        //     .get(format!("{pair}/sells", pair = self.pair))
        //     .await?;

        // let buys = buys.unwrap_or_default();
        // let sells = sells.unwrap_or_default();

        let orderbook = OrderBook::default();

        // info!(
        //     "order book: buys:{buys}, sells:{sells}",
        //     buys = buys.len(),
        //     sells = sells.len(),
        // );

        Ok(orderbook)
    }

    async fn mark_processed(&mut self, _orders: &[OrderRequest]) -> Result<()> {
        todo!()
    }
}
