use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod transfer;
pub mod cli;
pub mod http;
pub mod config;
pub mod catalog;

const INFO: &str = "

Starting UPM Dataspace protocol implementation
Show some love on github.com
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