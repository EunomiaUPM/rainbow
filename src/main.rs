#![allow(unused_imports)]

use crate::db::get_db_connection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl};
use tracing::{info, Level};

pub mod auth;
pub mod catalog;
pub mod cli;
pub mod config;
pub mod db;
pub mod transfer;

const INFO: &str = "
----------
Starting Rainbow 🌈🌈
UPM Dataspace protocol implementation
Show some love on https://github.com/ging/rainbow
----------

";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging setup
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("{}", INFO);

    // Create databases
    // TODO take it to cli start - start different databases...

    // Cli parser
    cli::init_command_line().await?;

    Ok(())
}
