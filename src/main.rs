#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::db::get_db_connection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl};
use tracing::{info, Level};

pub mod auth;
pub mod catalog;
pub mod cli;
pub mod config;
pub mod contracts;
pub mod db;
pub mod fake_catalog;
pub mod fake_contracts;
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
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    info!("{}", INFO);
    cli::init_command_line().await?;

    Ok(())
}
