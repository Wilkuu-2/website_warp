mod handlers;
mod tera;
mod routes;
mod error;
mod snippets;
mod common;

use std::net::SocketAddr;
use clap::{Subcommand,Parser};


#[derive(Subcommand,Clone, Copy, Debug)]
enum Security {
    NoSec,
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    security: Option<Security>,
    #[arg()]
    ip: Option<SocketAddr>,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Cli::parse();
    let paths = crate::routes::routes();

    let security: Security   = match &args.security {Some(sec) => *sec, None => Security::NoSec,};
    let address: SocketAddr  = match &args.ip {Some(ip) => *ip, None => "0.0.0.0:8080".parse().unwrap()};

    match security {
        Security::NoSec => {
            warp::serve(paths).run(address).await;
        },
    }

    println!("Running on: {} !", address);
}
