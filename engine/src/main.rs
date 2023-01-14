use anyhow::Result;
use domain::env::Opt;
use engine::ExchangeService;
use structopt::StructOpt;
use tracing::error;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )?;

    let opt = Opt::from_args();

    let mut service = ExchangeService::new(&opt.ticker, &opt.redis)?;
    if let Err(e) = service.run() {
        error!("service error {e:?}", e = e)
    }

    Ok(())
}
