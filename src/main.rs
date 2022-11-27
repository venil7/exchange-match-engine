pub mod env;

use env::Opt;
use exchange_service::ExchangeService;
use structopt::StructOpt;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let opt = Opt::from_args();

    let mut service = ExchangeService::try_new(&opt.pair, &opt.redis).await?;
    loop {
        let res = service.next().await?;
        info!("--> {:?}", res);
    }

    Ok(())
}
