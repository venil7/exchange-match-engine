use super::{OrderProvider, RedisProvider};
use crate::domain::{OrderBook, OrderRequest};
use anyhow::Result;
use tracing::error;

pub struct ExchangeService<Provider: OrderProvider> {
    pub pair: String,
    pub book: OrderBook,
    provider: Provider,
}

impl ExchangeService<RedisProvider> {
    pub async fn try_new(pair: &str, url: &str) -> Result<Self> {
        let mut provider = RedisProvider::try_new(pair, url).await?;
        let book = provider.load_order_book().await?;
        Ok(Self {
            pair: pair.into(),
            book,
            provider,
        })
    }

    async fn match_order(&mut self, order: OrderRequest) -> Result<()> {
        if let Some(orders) = self.book.match_order(order) {
            self.provider.mark_processed(&orders).await?;
        };
        // self.provider.save_order_book(&self.book).await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            match self.provider.next_order().await {
                Ok(order) => {
                    self.match_order(order).await?;
                }
                Err(err) => error!("{err}", err = err),
            }
        }
    }
}
