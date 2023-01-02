use anyhow::Result;
use exchange::{domain::Opt, server::create_api};
use structopt::StructOpt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )?;

    let opt = Opt::from_args();
    let api = create_api(&opt)?;

    warp::serve(api.or_else(|_| async { Err(warp::reject::not_found()) }))
        .run(opt.listen)
        .await;

    Ok(())
}
