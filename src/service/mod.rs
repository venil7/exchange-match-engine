use anyhow::Result;
use redis::{aio::AsyncStream, AsyncCommands, Client};
use sorted_vec::SortedVec;
use std::pin::Pin;
use tracing::info;

use crate::domain::order::OrderRequest;

pub struct ExchangeService {
    pub pair: String,
    pub connection: redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
    pub buys: SortedVec<OrderRequest>,
    pub sells: SortedVec<OrderRequest>,
}

impl ExchangeService {
    pub async fn try_new(pair: &str, url: &str) -> Result<Self> {
        let connection = Client::open(url)?.get_async_connection().await?;

        info!("connected to {url}", url = url);

        Ok(ExchangeService {
            pair: pair.into(),
            connection,
            buys: Default::default(),
            sells: Default::default(),
        })
    }

    async fn next(&mut self) -> Result<String> {
        let (_, val): (String, String) = self.connection.blpop(&self.pair, 0).await?;
        Ok(val)
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let res = self.next().await?;
            info!("--> {:?}", res);
        }
        #[allow(unreachable_code)]
        Ok(())
    }
}
