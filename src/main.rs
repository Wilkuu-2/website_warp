mod handlers;
mod tera;
mod routes;
mod error;
mod snippets;

use lets_encrypt_warp::lets_encrypt;
use warp::Filter;
use std::net::SocketAddr;
use clap::{Subcommand,Parser,Args};
use std::path::PathBuf;


#[derive(Args, Debug)]
struct LetsEncryptArgs {
    email: String,
    domain: String,
}

#[derive(Args, Debug)]
struct TlsArgs {
    cert: PathBuf,
    key: PathBuf,
}

#[derive(Subcommand, Debug)]
enum Security {
    NoSec,
    LetsEncrypt(LetsEncryptArgs),
    Tls(TlsArgs),

}

#[derive(Parser, Debug)]
struct Cli {
    ip: SocketAddr,
    #[command(subcommand)]
    security: Security,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let paths = crate::routes!();

    match &args.security {
        Security::NoSec => {
            warp::serve(paths).run(args.ip).await;
        },
        Security::Tls(tlsa) => {
            warp::serve(paths).tls().cert_path(&tlsa.cert).key_path(&tlsa.key).run(args.ip).await; 
        },
        Security::LetsEncrypt(lea) => {
            lets_encrypt(paths, &lea.email, &lea.domain).await.unwrap();
        },
    }
}
