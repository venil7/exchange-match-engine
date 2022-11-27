pub mod exchange;
use std::pin::Pin;

use anyhow::Result;
use redis::{aio::AsyncStream, AsyncCommands, Client};
use tracing::info;

pub struct ExchangeService {
    pub pair: String,
    pub connection: redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
}

impl ExchangeService {
    pub async fn try_new(pair: &str, url: &str) -> Result<Self> {
        let connection = Client::open(url)?.get_async_connection().await?;

        info!("connected to {url}", url = url);

        Ok(ExchangeService {
            pair: pair.into(),
            connection,
        })
    }

    pub async fn next(&mut self) -> Result<String> {
        let (_, val): (String, String) = self.connection.blpop(&self.pair, 0).await?;
        Ok(val)
    }
}
