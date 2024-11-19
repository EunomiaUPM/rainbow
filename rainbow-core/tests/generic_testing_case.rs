// mod utils;

use tracing_test::traced_test;

#[path = "utils.rs"]
mod utils;

#[traced_test]
#[tokio::test]
pub async fn transfer_pull_full_case() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        catalog_id,
        dataservice_id,
        agreement_id,
        consumer_pid,
        consumer_callback_address,
        callback_id,
    ) = utils::setup_test_env("a").await?;

    // Your test here please....
    assert_eq!(1, 1);

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}
