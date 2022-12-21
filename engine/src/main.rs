use exchange::{domain::env::Opt, service::ExchangeService};
use structopt::StructOpt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let opt = Opt::from_args();

    let mut service = ExchangeService::try_new(&opt.ticker, &opt.redis).await?;

    service.run().await?;

    Ok(())
}
