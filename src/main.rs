mod handlers;
mod tera;
mod routes;
mod error;
mod snippets;

use std::{net::SocketAddr};
use clap::{Subcommand,Parser};
use std::env; 
use sea_orm::{DatabaseConnection, Database, JsonValue, Statement, DbBackend, FromQueryResult};


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
    let db_user = env::var("MYSQL_USERNAME").unwrap();
    let db_pass = env::var("MYSQL_PASSWORD").unwrap();
    let db_name = env::var("MYSQL_DATABASE").unwrap();

    let db_url = format!("mysql://{db_user}:{db_pass}@mysql/{db_name}");
    println!("Trying to connect to: {}", db_url);

    let db: DatabaseConnection = Database::connect(db_url).await.unwrap();

    let dn_table: Vec<JsonValue> = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::MySql, 
            r#"SELECT * FROM deez_nuts"#, 
            [], )
        ).all(&db).await.unwrap();

    println!("Deez nuts table: \n{:?}", dn_table);
    
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
