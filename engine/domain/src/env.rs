use std::net::SocketAddr;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "exchange-matcher", about = "usage")]
pub struct Opt {
    #[structopt(short, long, default_value = "abc")]
    pub ticker: String,

    #[structopt(short, long, default_value = "redis://127.0.0.1")]
    pub redis: String,

    #[structopt(short, long, default_value = "0.0.0.0:4041")]
    pub listen: SocketAddr,
}
