use super::{OrderBookProvider, RedisProvider};
use anyhow::Result;
use domain::{Order, OrderBook};
use tracing::{error, trace};

pub struct ExchangeService<Provider: OrderBookProvider + ?Sized> {
    pub ticker: String,
    pub book: OrderBook,
    provider: Provider,
}

impl ExchangeService<RedisProvider<Box<redis::Connection>>> {
    pub fn new(ticker: &str, url: &str) -> Result<Self> {
        let mut provider = RedisProvider::new(ticker, url)?;
        let book = provider.load_order_book()?;
        Ok(Self {
            ticker: ticker.into(),
            book,
            provider,
        })
    }

    fn process_order(&mut self, order: Order) -> Result<()> {
        trace!("incoming order: {order}", order = order);
        self.book.add_order(order);
        let txs = self.book.match_and_process_orders();
        self.provider.mark_processed(&txs)?;
        self.provider.save_order_book(&self.book)?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.provider.dequeue() {
                Ok(order) => {
                    self.process_order(order)?;
                }
                Err(err) => error!("{err}", err = err),
            }
        }
    }
}
