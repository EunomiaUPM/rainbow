#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::integration::utils::{cleanup_test_env, get_json_file, load_env_file, setup_agreements_and_datasets, setup_test_env};
use tracing_test::traced_test;


#[traced_test]
#[tokio::test]
pub async fn transfer_push_full_case() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
    ) = setup_test_env().await?;

    // logic here

    cleanup_test_env(provider_server, consumer_server).await
}
