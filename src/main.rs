use tracing::{info, Level};

pub mod transfer;
pub mod cli;
pub mod http;
pub mod config;
pub mod catalog;

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

    // Cli parser
    cli::init_command_line().await?;

    Ok(())
}