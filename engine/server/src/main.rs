use anyhow::Result;
use api::create_api;
use domain::Opt;
use structopt::StructOpt;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;
use warp::Filter;

mod api;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )?;

    let opt = Opt::from_args();
    let api = create_api(&opt)?;

    warp::serve(api).run(opt.listen).await;

    Ok(())
}
