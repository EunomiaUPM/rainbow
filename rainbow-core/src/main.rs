#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]


use tracing::{info, Level};
use rainbow_core::setup;

const INFO: &str = r"
----------
 ____    __    ____  _  _  ____  _____  _    _
(  _ \  /__\  (_  _)( \( )(  _ \(  _  )( \/\/ )
 )   / /(__)\  _)(_  )  (  ) _ < )(_)(  )    (
(_)\_)(__)(__)(____)(_)\_)(____/(_____)(__/\__)

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
    setup::commands::init_command_line().await?;

    Ok(())
}
