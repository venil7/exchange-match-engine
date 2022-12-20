use super::{OrderProvider, RedisProvider};
use crate::domain::{OrderBook, OrderRequest};
use anyhow::Result;
use tracing::{error, trace};

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

    async fn process_order(&mut self, order: OrderRequest) -> Result<()> {
        trace!("incoming order: {order}", order = order);
        self.book.add_order(order);
        let txs = self.book.match_and_process_orders();
        self.provider.mark_processed(&txs).await?;
        self.provider.save_order_book(&self.book).await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            match self.provider.next_order().await {
                Ok(order) => {
                    self.process_order(order).await?;
                }
                Err(err) => error!("{err}", err = err),
            }
        }
    }
}
