use tracing::info;
use rainbow_catalog::setup::init_command_line;

const INFO: &str = r"
----------
 ____    __    ____  _  _  ____  _____  _    _
(  _ \  /__\  (_  _)( \( )(  _ \(  _  )( \/\/ )
 )   / /(__)\  _)(_  )  (  ) _ < )(_)(  )    (
(_)\_)(__)(__)(____)(_)\_)(____/(_____)(__/\__)

Starting Rainbow Catalog Server 🌈🌈
UPM Dataspace protocol implementation
Show some love on https://github.com/ging/rainbow
----------

";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();
    info!("{}", INFO);
    init_command_line().await.expect("TODO: panic message");
}
