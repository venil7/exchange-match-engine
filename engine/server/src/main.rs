use std::time::Duration;

use anyhow::Result;
use api::create_api;
use domain::Opt;
use lazy_static::lazy_static;
use structopt::StructOpt;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod handlers;
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

lazy_static! {
    pub static ref REDIS_POOL: r2d2::Pool<redis::Client> = {
        const CACHE_POOL_MAX_OPEN: u32 = 16;
        const CACHE_POOL_MIN_IDLE: u32 = 8;
        const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

        let opt = Opt::from_args();
        info!("{:?}", opt);
        let client = redis::Client::open(opt.redis).unwrap();

        let pool = r2d2::Pool::builder()
            .max_size(CACHE_POOL_MAX_OPEN)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .min_idle(Some(CACHE_POOL_MIN_IDLE))
            .build(client)
            .unwrap();
        pool
    };
}
